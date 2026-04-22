
use stateful_search_engine::run;


fn main() {
    if let Err(err) = run(){
        println!("Error: {}",err)
    };
}
