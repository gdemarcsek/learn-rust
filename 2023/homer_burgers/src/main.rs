use std::cmp::max;
use std::collections::HashMap;

#[derive(Debug)]
struct BurgerProblem {
    m: u32,
    n: u32,
    t: u32,
}

#[derive(Debug)]
struct BurgerSolution {
    burger_minutes: u32,
    beer_minutes: u32,
}

impl BurgerProblem {
    fn solve_exactly_with_cache(&self, cache : &mut HashMap<u32, Option<u32>>) -> Option<u32> {
        if self.t == 0 {
            return Some(0);
        }

        if cache.contains_key(&self.t) {
            return *cache.get(&self.t).unwrap();
        }

        let mut first : Option<u32> = None;
        let mut second : Option<u32> = None;

        if self.t >= self.m {
            first = BurgerProblem{ m: self.m, n: self.n, t: self.t - self.m }.solve_exactly_with_cache(cache);
        }

        if self.t >= self.n {
            second = BurgerProblem{ m: self.m, n: self.n, t: self.t - self.n }.solve_exactly_with_cache(cache);
        }

        if first.is_none() && second.is_none() {
            cache.insert(self.t, None);
            None
        } else {
            let result = Some(max(first.unwrap_or(0), second.unwrap_or(0)) + 1);
            cache.insert(self.t, result);
            result
        }
    }

    fn solve(&self) -> BurgerSolution {
        let mut cache: HashMap<u32, Option<u32>> = HashMap::new();
        match self.solve_exactly_with_cache(&mut cache)  {
            Some(solution) => BurgerSolution{burger_minutes: solution, beer_minutes: 0},
            None => {
                let mut t = self.t - 1;
                let mut result = BurgerProblem{m: self.m, n: self.n, t: t}.solve_exactly_with_cache(&mut cache);
                while result.is_none() {
                    t -= 1;
                    result = BurgerProblem{m: self.m, n: self.n, t: t}.solve_exactly_with_cache(&mut cache);
                }
                BurgerSolution{burger_minutes: result.unwrap(), beer_minutes: self.t - t}
            }
        }
    }
}

fn main() {
    println!("{:?}", BurgerProblem{m: 4, n: 2, t: 54}.solve());
}
