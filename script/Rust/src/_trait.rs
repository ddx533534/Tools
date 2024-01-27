//### 特征,定义了一组可以被共享的行为，只要实现了特征，你就能使用这组行为。

pub trait Summary{
    fn summary(&self);
}


pub struct Animal{
    name:String,
    catogery:String
}
impl Animal {
    pub fn new(name:String, catogery:String) -> Animal{
        Animal{
            name:name,
            catogery:catogery
        }
    }
}

impl Summary for Animal{
    fn summary(&self){
        println!("animal name {:?},catogery: {:?}",self.name,self.catogery);
    }
}

pub struct Sport{
    name:String,
    catogery:String
}
impl Summary for Sport{
    fn summary(&self){
        println!("animal name {:?},catogery: {:?}",self.name,self.catogery);
    }
}

