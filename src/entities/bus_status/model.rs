



use crate::handlers::db::pg::establish as pg;
use r2d2_postgres::postgres::Row;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Duration, Offset, Utc};
use chrono::prelude::*;




#[derive(Serialize, Deserialize)]
pub struct BusStatus{
    pub device_id: String,
    pub total_time_on: i64,
    pub total_time_off: i64,
    pub number_of_status_changes: i16,
}

impl BusStatus{
    


    pub async fn process_switch_changes(fetch_all_status_in_one_day: Vec<Row>) -> i16{
        if fetch_all_status_in_one_day.len() == 0{
            0 //-- no record found
        } else{
            let mut n_changes_in_one_day: i16 = 0;
            let mut first_status: i16 = fetch_all_status_in_one_day[0].get(0);
            for idx in 1..fetch_all_status_in_one_day.len(){
                let status: i16 = fetch_all_status_in_one_day[idx].get(0);
                if status != first_status{
                    n_changes_in_one_day += 1;
                    first_status = status;
                }
            }
            n_changes_in_one_day
        }
    }
    

    pub async fn calculate_total_time_on(id: i32 , from_time: String , to_time: String) -> Result<Self, Box<dyn std::error::Error>>{
        let pool = pg::connection("inobi").await.unwrap();
        let mut conn = pool.get().unwrap(); //-- getting a connection from the created pool
        


        

        // =====================================
        // parsing from and to time using chrono
        // =====================================
        let from_time_parsed = DateTime::parse_from_rfc3339(from_time.as_str()).unwrap();
        let to_time_parsed = DateTime::parse_from_rfc3339(to_time.as_str()).unwrap();
        let days = (to_time_parsed.date() - from_time_parsed.date()).num_days();
        




        // ====================
        // extracting timezeone
        // ====================
        let from_timestamp = from_time_parsed.timestamp();
        let to_timestamp = to_time_parsed.timestamp();
        let timezone = from_time_parsed.offset().fix().local_minus_utc() / 3600;
       
       




        // ========================================
        // fetching device_id from transports table
        // ========================================
        let fetch_device_id_statement = conn.prepare("select device_id from transports where id = ($1)").unwrap();
        let device_uniqueid = conn.query(&fetch_device_id_statement, &[&id]).unwrap();
        let device_id: String = device_uniqueid[0].get(0);
        




        // ====================================================
        // calculating number of all status changes in all days
        // ====================================================
        let mut all_switch_changes_in_all_days = 0;
        let from_time_parsed_string = from_time_parsed.format("%Y-%m-%d %H:%M:%S").to_string();
        let mut today = Utc.datetime_from_str(from_time_parsed_string.as_str(), "%Y-%m-%d %H:%M:%S").unwrap();
        for _ in 0..days{
            let next_day = today + Duration::hours(24);
            let today_timestamp = today.timestamp();
            let next_day_timestamp = next_day.timestamp();
            today = next_day;
            let all_status_in_one_day_stmt = conn.prepare("select status from bus_info where device_id = ($1) and time between ($2) and ($3)").unwrap();
            let fetch_all_status_in_one_day = conn.query(&all_status_in_one_day_stmt, &[&device_id, &today_timestamp, &next_day_timestamp]).unwrap();
            println!("[+] Time after fetch is {}", chrono::Local::now().naive_local());
           let switch_changes_in_one_day = BusStatus::process_switch_changes(fetch_all_status_in_one_day).await;
            if switch_changes_in_one_day != 0{
                all_switch_changes_in_all_days += switch_changes_in_one_day;
            } else{
                // no record found with this time
                // ...
            }
        }





        // ============================================
        // calculating total time on and total time off
        // ============================================
        let first_match_with_from_stmt = conn.prepare("select time from bus_info where time > ($1) and device_id = ($2)").unwrap();
        let fetch_first_match_with_from = conn.query(&first_match_with_from_stmt, &[&from_timestamp, &device_id]).unwrap();
        let first_match_with_from = if fetch_first_match_with_from.len() == 0{
            0
        } else{
            fetch_first_match_with_from[0].get(0)
        };


        let first_match_with_to_stmt = conn.prepare("select time from bus_info where time < ($1) and device_id = ($2) order by time desc;").unwrap();
        let fetch_first_match_with_to = conn.query(&first_match_with_to_stmt, &[&to_timestamp, &device_id]).unwrap();
        let first_match_with_to = if fetch_first_match_with_to.len() == 0{
            0
        } else{
            fetch_first_match_with_to[0].get(0)
        };

        let from_total_time_on_stmt = conn.prepare("select total_time_on from bus_info where time = ($1) and device_id = ($2)").unwrap();
        let fetch_from_total_time_on = conn.query(&from_total_time_on_stmt, &[&first_match_with_from, &device_id])?;
        let from_total_time_on = if fetch_from_total_time_on.len() == 0{
            0
        } else{
            fetch_from_total_time_on[0].get(0)
        };



        let to_total_time_on_stmt = conn.prepare("select total_time_on from bus_info where time = ($1) and device_id = ($2)").unwrap();
        let fetch_to_total_time_on = conn.query(&to_total_time_on_stmt, &[&first_match_with_to, &device_id])?;
        let to_total_time_on = if fetch_to_total_time_on.len() == 0{
            0
        } else{
            fetch_to_total_time_on[0].get(0)
        };


        if first_match_with_from == 0 || first_match_with_to == 0{
            Ok(
                BusStatus{
                    device_id: device_id.to_string(),
                    total_time_on: 0,
                    total_time_off: 0,
                    number_of_status_changes: all_switch_changes_in_all_days
                }
            )
        } else{
            let total_time_on = to_total_time_on - from_total_time_on;
            let total_time_off = (first_match_with_to - first_match_with_from) - total_time_on;
            if total_time_off < 0{
                Ok(
                    BusStatus{
                        device_id: device_id.to_string(),
                        total_time_on: 0,
                        total_time_off: 0,
                        number_of_status_changes: all_switch_changes_in_all_days
                    }
                )
            } else{
                Ok(
                    BusStatus{
                        device_id: device_id.to_string(),
                        total_time_on,
                        total_time_off,
                        number_of_status_changes: all_switch_changes_in_all_days
                    }
                )
            }
        }
    
    }

}








