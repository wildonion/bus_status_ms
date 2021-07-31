








use actix_web::{get, web, HttpResponse, Result};
use super::model::BusStatus;






#[get("/avl/api/reports/status/{id}/{from_time}/{to_time}")]
async fn calculate(web::Path((id, from_time, to_time)): web::Path<(i32, String, String)>) -> Result<HttpResponse, ()>{
    match BusStatus::calculate_total_time_on(id, from_time, to_time).await{
        Ok(total_time_on) => Ok(HttpResponse::Ok().json(total_time_on)),
        Err(e) => Err(()), //-- cause we're returning () inside the failure Result  
    }
}





pub fn init_service(config: &mut web::ServiceConfig){
    config.service(calculate);
}