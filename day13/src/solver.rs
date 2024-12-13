use crate::puzzle_input::ClawMachine;

pub trait Solver {
    fn solve(&self) -> Option<(i64, i64)>;
}

impl Solver for ClawMachine {
    fn solve(&self) -> Option<(i64, i64)> {
        // Solve this system of equations for Sa and Sb (assuming all variables != 0, so no degenerate cases):
        //    Sa*Ax + Sb*Bx = Tx
        //    Sa*Ay + Sb*By = Ty
        // Solve for Sa:
        //    Sa = (Tx - Sb*Bx) / Ax
        // Substitute Sa in the second equation
        //    (Tx/Ax - Sb*Bx/Ax)*Ay + Sb*By = Ty
        //    Tx*Ay/Ax - Sb*Bx*Ay/Ax + Sb*By = Ty
        // Multiply by Ax to get rid of the division
        //    Tx*Ay - Sb*Bx*Ay + Sb*By*Ax = Ty*Ax
        //    Sb*By*Ax - Sb*Bx*Ay =  Ty*Ax - Tx*Ay
        //    Sb*(By*Ax - Bx*Ay) = Ty*Ax - Tx*Ay
        // Solve for Sb
        //   Sb = (Ty*Ax - Tx*Ay) / (By*Ax - Bx*Ay)
        // Then solve for Sa
        //    Sa = (Tx - Sb*Bx) / Ax

        // A solution exists iff (By*Ax - Bx*Ay) divides (Tx*Ay - Ty*Ax)

        let det = self.b.y * self.a.x - self.b.x * self.a.y;
        assert!(det != 0);

        if (self.target.x * self.a.y - self.target.y * self.a.x) % det != 0 {
            return None;
        }

        let b = (self.target.y * self.a.x - self.target.x * self.a.y) / det;
        if (self.target.x - b * self.b.x) % self.a.x != 0 {
            // Division won't find an integer solution.
            return None;
        }
        let a = (self.target.x - b * self.b.x) / self.a.x;

        let solution_xy = (a, b);

        // Verify the solution satisfied both axes.
        assert!(solution_xy.0 * self.a.x + solution_xy.1 * self.b.x == self.target.x);
        assert!(solution_xy.0 * self.a.y + solution_xy.1 * self.b.y == self.target.y);

        Some(solution_xy)
    }
}
