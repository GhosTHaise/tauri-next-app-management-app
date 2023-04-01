use reqwest::Url;
use super::forecast::_struct::Forecast;
struct FetchError {
    pub message : String
}
pub struct Api;

impl Api{
    fn get(city : &String,country_code : &String) -> Result<Forecast,FetchError> 
    {
        
        todo!()
    }
}