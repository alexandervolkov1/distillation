#[derive(Clone, Debug)]
pub struct Task {
    pub v_0: f64,
    pub v_pot: f64,
    pub v_sec: f64,
    pub v_def: f64,
    pub flow: f64,
    pub plate_count: usize,
    pub alpha: f64,
    pub drop_count: usize,
    pub sum_removed_impurity: f64,
    pub times: Vec<f64>,
    pub factors: Vec<f64>,
    pub removed_impurities: Vec<f64>,
    f_0: f64,
    s_0: f64,
    ln_s_0: f64,
}

impl Task {
    pub fn new(
        v_0: f64, v_sec: f64, v_def: f64,
        flow: f64, plate_count: usize, alpha: f64,
    ) -> Self {
        let mut temp_task = Self {
            v_0,
            v_pot: v_0 - v_sec - v_def,
            v_sec,
            v_def,
            flow,
            plate_count,
            alpha,
            drop_count: 0,
            sum_removed_impurity: 0.0,
            times: Vec::new(),
            factors: Vec::new(),
            removed_impurities: Vec::new(),
            f_0: alpha.powi(plate_count as i32),
            s_0: 0.0,
            ln_s_0: 0.0,
        };
        temp_task.s_0 = temp_task.s(temp_task.f_0);
        temp_task.ln_s_0 = temp_task.s_0.ln();
        temp_task
    }

    fn r(&self, f: f64) -> f64 {
        self.v_def*f / (self.v_pot + self.v_sec*(f - 1.0)/f.ln() + self.v_def*f)
    }

    fn s(&self, f: f64) -> f64 {
        (self.v_pot + self.v_sec + self.v_def)*f 
            / (self.v_pot + self.v_sec*(f - 1.0)/f.ln() + self.v_def*f)
    }

    fn get_time(&self, f: f64) -> f64 {
        self.alpha/(self.alpha - 1.0)/self.flow
            * ((self.s_0 - 1.0)/(self.s_0 - self.s(f))).ln()
                * (self.v_sec*((self.s_0 - 1.0)/self.ln_s_0 - 1.0) + self.v_def*(self.s_0 - 1.0))
    }

    pub fn solve(&self, time: f64) -> f64 {
        const EPS: f64 = 0.00001;
        const MAX_ITER: usize = 100;
        let mut left = 1.0 + EPS;
        let mut right = self.alpha.powi(self.plate_count as i32) - EPS;
        let mut middle;

        for _ in 0..MAX_ITER {
            middle = 0.5 * (left + right);
            let middle_time = self.get_time(middle);

            if (middle_time - time).abs() < EPS {
                return middle;
            }
            if middle_time > time {
                right = middle;
            } else {
                left = middle;
            }
        }
        0.5 * (left + right)
    }

    pub fn do_drop_when_factor(&mut self, target_factor: f64) -> &mut Self {
        assert!(self.v_pot > self.v_def, "The substance in the pot ran out!");
        self.factors.push(target_factor);
        let f_fall = target_factor.powf((self.v_sec - self.v_def)/self.v_sec);
        self.factors.push(f_fall);

        if self.times.is_empty() {
            self.times.push(self.get_time(target_factor));
        } else {
            self.times.push(
                self.get_time(target_factor)
                    - self.get_time(self.factors[self.factors.len() - 3])
                        + self.times.last().unwrap()
            );
        }

        self.removed_impurities.push((1.0 - self.sum_removed_impurity) * self.r(target_factor));
        self.v_pot -= self.v_def;
        
        self.sum_removed_impurity += self.removed_impurities[self.drop_count];
        self.drop_count += 1;

        self
    }

    pub fn do_drop_in_time(&mut self, time: f64) -> &mut Self {
        let temp_time = if self.times.is_empty() {
            time
        } else {
            time + self.get_time(*self.factors.last().unwrap())
        };
        
        let target_factor = self.solve(temp_time);
        
        self.do_drop_when_factor(target_factor)       
    }

    pub fn do_drop_while(&mut self, start_period: f64, time_between_drops: f64, needed_fraction: f64) -> &mut Self {

        self.do_drop_in_time(start_period);

        while self.sum_removed_impurity < needed_fraction {
        self.do_drop_in_time(time_between_drops);
        }
        self
    }

    pub fn productivity(&self) -> f64 {
        self.v_pot / self.times.last().unwrap()
    }

    pub fn product_yield(&self) -> f64 {
        self.v_pot / self.v_0
    }

    pub fn efficiency(&self) -> f64 {
        self.productivity() * self.product_yield()
    }
}
