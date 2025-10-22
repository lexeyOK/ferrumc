pub trait AlgebraicExt<Rhs> {
    type Output;

    fn algebraic_add(self, rhs: Rhs) -> Self::Output;
    fn algebraic_sub(self, rhs: Rhs) -> Self::Output;
    fn algebraic_mul(self, rhs: Rhs) -> Self::Output;
    fn algebraic_div(self, rhs: Rhs) -> Self::Output;
}

pub trait AlgebraicDot<Rhs> {
    type Output;
    fn algebraic_dot(self, rhs: Rhs) -> Self::Output;
}
use bevy_math::DVec3;

impl AlgebraicDot<DVec3> for DVec3 {
    type Output = f64;

    fn algebraic_dot(self, rhs: DVec3) -> Self::Output {
        let DVec3 { x, y, z } = self.algebraic_mul(rhs);
        x.algebraic_add(y).algebraic_add(z)
    }
}

impl AlgebraicExt<DVec3> for DVec3 {
    type Output = DVec3;

    fn algebraic_add(self, rhs: DVec3) -> Self::Output {
        Self::Output {
            x: self.x.algebraic_add(rhs.x),
            y: self.y.algebraic_add(rhs.y),
            z: self.z.algebraic_add(rhs.z),
        }
    }

    fn algebraic_sub(self, rhs: DVec3) -> Self::Output {
        Self::Output {
            x: self.x.algebraic_sub(rhs.x),
            y: self.y.algebraic_sub(rhs.y),
            z: self.z.algebraic_sub(rhs.z),
        }
    }

    fn algebraic_mul(self, rhs: DVec3) -> Self::Output {
        Self::Output {
            x: self.x.algebraic_mul(rhs.x),
            y: self.y.algebraic_mul(rhs.y),
            z: self.z.algebraic_mul(rhs.z),
        }
    }

    fn algebraic_div(self, rhs: DVec3) -> Self::Output {
        Self::Output {
            x: self.x.algebraic_div(rhs.x),
            y: self.y.algebraic_div(rhs.y),
            z: self.z.algebraic_div(rhs.z),
        }
    }
}
