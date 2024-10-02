
// Rc<T> 使用场景： 需要在heap上分配数据，这些数据被程序的多个部分读取（只读），但在编译阶段无法确定哪个部分最后使用这些数据
use std::rc::Rc;

#[derive(Debug)]
pub enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
}