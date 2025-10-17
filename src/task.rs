#[derive(Clone, Debug)]
pub struct Task {
    v_pot: f64,
    v_sec: f64,
    v_def: f64,
    flow: f64,
    pub plate_count: usize,
    pub alpha: f64,
    drop_count: usize,
    sum_removed_impurity: f64,
    times: Vec<f64>,
    factors: Vec<f64>,
    removed_impurities: Vec<f64>,
}

impl Task {
    pub fn new(
        v_pot: f64, v_sec: f64, v_def: f64,
        flow: f64, plate_count: usize, alpha: f64,
    ) -> Self {
        Self {
            v_pot, v_sec, v_def,
            flow, plate_count, alpha,
            drop_count: 0,
            sum_removed_impurity: 0.0,
            times: Vec::new(),
            factors: Vec::new(),
            removed_impurities: Vec::new(),
        }
    }

    fn r(&self, f: f64) -> f64 {
        self.v_def*f / (self.v_pot + self.v_sec*(f - 1.0)/f.ln() + self.v_def*f)
    }

    fn s(&self, f: f64) -> f64 {
        (self.v_pot + self.v_sec + self.v_def)*f 
            / (self.v_pot + self.v_sec*(f - 1.0)/f.ln() + self.v_def*f)
    }

    fn get_time(&self, f: f64) -> f64 {
        let f_0 = self.alpha.powf(self.plate_count as f64);
        let s_0 = self.s(f_0);
        self.alpha/(self.alpha - 1.0)/self.flow
            * ((s_0 - 1.0)/(s_0 - self.s(f))).ln()
                * (self.v_sec*((s_0 - 1.0)/s_0.ln() - 1.0) + self.v_def*(s_0 - 1.0))
    }

    pub fn do_drop_when_factor(&mut self, f_lim: f64) -> &mut Self {
        self.factors.push(f_lim);
        let f_fall = f_lim.powf((self.v_sec - self.v_def)/self.v_sec);
        self.factors.push(f_fall);

        if self.times.is_empty() {
            self.times.push(self.get_time(f_lim));
        } else {
            self.times.push(
                self.get_time(f_lim)
                    - self.get_time(self.factors[self.factors.len() - 3])
                        + self.times.last().unwrap()
            );
        }

        self.removed_impurities.push((1.0 - self.sum_removed_impurity) * self.r(f_lim));
        self.v_pot -= self.v_def;
        
        self.sum_removed_impurity += self.removed_impurities[self.drop_count];
        self.drop_count += 1;

        self
    }

    pub fn do_drop_in_time(&mut self, time: f64) -> &mut Self {
        let temp_time = if self.times.is_empty() {
            time
        } else {
            time + self.get_time(self.factors.last().unwrap().clone())
        };
        
        let f_lim = self.solve(temp_time);
        
        self.do_drop_when_factor(f_lim)       
    }

    pub fn solve(&self, time: f64) -> f64 {
        const EPS: f64 = 0.0001;
        let mut left_border = 1.0 + EPS;
        let mut right_border = self.alpha.powf(self.plate_count as f64) - EPS;
        let mut left_time;
        let mut temp = 0.0;
        let mut temp_time;

        while right_border - left_border > EPS {
            temp = left_border + (right_border - left_border)/2.0;
            temp_time = self.get_time(temp);
            left_time = self.get_time(left_border);

            if (left_time < time && temp_time > time)
                || (left_time > time && temp_time < time) {
                right_border = temp;
            } else {
                left_border = temp;
            }
        }
        temp
    }
}
