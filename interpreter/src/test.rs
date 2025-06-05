trait Pastry {
    fn accept(&self, visitor: &impl PastryVisitor);
}

struct Beignet {

}
struct Cruller {

}

impl Pastry for Beignet {
    fn accept(&self, visitor: &impl PastryVisitor) {
        visitor.visit_beignet(self);
    }
}
impl Pastry for Cruller {
    fn accept(&self, visitor: &impl PastryVisitor) {
        visitor.visit_cruller(self);
    }
}

trait PastryVisitor {
    fn visit_beignet(&self, beignet: &Beignet);
    fn visit_cruller(&self, cruller: &Cruller);
}

struct Printer {

}

impl PastryVisitor for Printer {
    fn visit_beignet(&self, beignet: &Beignet) {
        beignet.accept(self);
    }
    fn visit_cruller(&self, cruller: &Cruller) {
        
    }
}