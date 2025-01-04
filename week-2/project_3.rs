fn main(){
    let p:f64=210_000.0;
    let r:f64=5.0;
    let t:f64=3.o;

    // compound interest depreciation
    let a:f64=p*(1.0-(r/100.0)).powf(t);
    let cid=a-p;
    println!("{}",cid);
}