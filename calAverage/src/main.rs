mod lib;
fn main(){
    let v = Vec::new();
    let mut a = lib::AveragedCollection{list: v,average:0 as f64};
    a.add(2);
    a.add(3);
    a.add(4);
    a.remove();
    println!("{}",a.average());
}