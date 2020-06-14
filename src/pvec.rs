use crate::Pattern;

pub struct PatternVec(Vec<Pattern>);
impl PatternVec {
    pub fn get(self) -> Vec<Pattern> {
        self.0
    }
}
impl From<Vec<Pattern>> for PatternVec {
    fn from(v: Vec<Pattern>) -> PatternVec {
        PatternVec(v.into())
    }
}
impl From<()> for PatternVec {
    fn from(_: ()) -> PatternVec {
        PatternVec(vec![])
    }
}
impl<A1: Into<Pattern>> From<A1> for PatternVec {
    fn from(a1: A1) -> PatternVec {
        PatternVec(vec![a1.into()])
    }
}
impl<A1: Into<Pattern>> From<(A1, )> for PatternVec {
    fn from(a1: (A1, )) -> PatternVec {
        PatternVec(vec![a1.0.into()])
    }
}
impl<A1, A2> From<(A1, A2)> for PatternVec
where
    A1: Into<Pattern>,
    A2: Into<Pattern>,
{
    fn from(a: (A1, A2)) -> PatternVec {
        PatternVec(vec![a.0.into(), a.1.into()])
    }
}
impl<A1, A2, A3> From<(A1, A2, A3)> for PatternVec
where
    A1: Into<Pattern>,
    A2: Into<Pattern>,
    A3: Into<Pattern>,
{
    fn from(a: (A1, A2, A3)) -> PatternVec {
        PatternVec(vec![a.0.into(), a.1.into(), a.2.into()])
    }
}
impl<A1, A2, A3, A4> From<(A1, A2, A3, A4)> for PatternVec
where
    A1: Into<Pattern>,
    A2: Into<Pattern>,
    A3: Into<Pattern>,
    A4: Into<Pattern>,
{
    fn from(a: (A1, A2, A3, A4)) -> PatternVec {
        PatternVec(vec![a.0.into(), a.1.into(), a.2.into(), a.3.into()])
    }
}
impl<A1, A2, A3, A4, A5> From<(A1, A2, A3, A4, A5)> for PatternVec
where
    A1: Into<Pattern>,
    A2: Into<Pattern>,
    A3: Into<Pattern>,
    A4: Into<Pattern>,
    A5: Into<Pattern>,
{
    fn from(a: (A1, A2, A3, A4, A5)) -> PatternVec {
        PatternVec(vec![
            a.0.into(),
            a.1.into(),
            a.2.into(),
            a.3.into(),
            a.4.into(),
        ])
    }
}
impl<A1, A2, A3, A4, A5, A6> From<(A1, A2, A3, A4, A5, A6)> for PatternVec
where
    A1: Into<Pattern>,
    A2: Into<Pattern>,
    A3: Into<Pattern>,
    A4: Into<Pattern>,
    A5: Into<Pattern>,
    A6: Into<Pattern>,
{
    fn from(a: (A1, A2, A3, A4, A5, A6)) -> PatternVec {
        PatternVec(vec![
            a.0.into(),
            a.1.into(),
            a.2.into(),
            a.3.into(),
            a.4.into(),
            a.5.into(),
        ])
    }
}
impl<A1, A2, A3, A4, A5, A6, A7> From<(A1, A2, A3, A4, A5, A6, A7)> for PatternVec
where
    A1: Into<Pattern>,
    A2: Into<Pattern>,
    A3: Into<Pattern>,
    A4: Into<Pattern>,
    A5: Into<Pattern>,
    A6: Into<Pattern>,
    A7: Into<Pattern>,
{
    fn from(a: (A1, A2, A3, A4, A5, A6, A7)) -> PatternVec {
        PatternVec(vec![
            a.0.into(),
            a.1.into(),
            a.2.into(),
            a.3.into(),
            a.4.into(),
            a.5.into(),
            a.6.into(),
        ])
    }
}
impl<A1, A2, A3, A4, A5, A6, A7, A8> From<(A1, A2, A3, A4, A5, A6, A7, A8)> for PatternVec
where
    A1: Into<Pattern>,
    A2: Into<Pattern>,
    A3: Into<Pattern>,
    A4: Into<Pattern>,
    A5: Into<Pattern>,
    A6: Into<Pattern>,
    A7: Into<Pattern>,
    A8: Into<Pattern>,
{
    fn from(a: (A1, A2, A3, A4, A5, A6, A7, A8)) -> PatternVec {
        PatternVec(vec![
            a.0.into(),
            a.1.into(),
            a.2.into(),
            a.3.into(),
            a.4.into(),
            a.5.into(),
            a.6.into(),
            a.7.into(),
        ])
    }
}
