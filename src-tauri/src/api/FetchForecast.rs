use std::{thread, result};
use super::forecast::_struct::Forecast;
use ureq::json;
struct FetchError {
    pub message : String
}
pub struct Api;

impl Api{

    fn prepare_url(&self,city : &String,country_code : &String) -> String {
        format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&appid=0cd2598c06685a9e3497c04dccea7395",city,country_code)
    }

    fn get(&self,city : &String,country_code : &String) -> Result<Forecast,FetchError> 
    {
        let url = self.prepare_url(city, country_code);
        let handle = thread::spawn(move||{
            let response = ureq::get(&url).call().unwrap();
            if(response.status() == 200){
                Ok(response)
            }else{
                Err(FetchError{
                    message: "Unable to fetch api".to_string()
                })
            }
        });
        let result = handle.join().unwrap();
        match result {
             Ok(response) => {
                Ok(response.into_json::<Forecast>().unwrap())
             }
             Err(error)=>{
                Err(error)
             }
        }
    }
}