use api::get_articless;

fn main(){
    let temp=get_articless().unwrap();
    let temp2 = temp.articles();
    println!("{:?}",temp2)
   

}