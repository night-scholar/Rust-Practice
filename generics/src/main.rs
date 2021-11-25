struct Point <T,U>{
    x: T,
    y: U,
}


impl <T,U> Point <T,U>{
    fn mixup <V,W>(self,point: Point<V,W>) -> Point<T,W>{
        Point{
            x:self.x,
            y:point.y,
        }
    }
} 

fn main(){
    let point1 = Point{x:1,y:2};
    let point2 = Point{x:"three",y:"four"};
    let point3 = point1.mixup(point2);
    println!("{},{}",point3.x,point3.y);

}