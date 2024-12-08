
pub fn _main() {
    // Part 2 works by adding do-support ;)
    _part1_2();
}

pub enum _MulStates {
    Skip,
    M,
    Mu,
    Mul,
    MulPar,
    MulParNums,
    MulParNumsC,
    MulParNumsCommaNums
}
pub enum _DoStates {
    D,
    Do,
    DoPar,
    Skip,
    Don,
    Donn,
    Donnt,
    DonntPar
}

pub struct _CollectMuls {
    pub mul_state: _MulStates,
    pub do_state: _DoStates,
    pub num1: String,
    pub num2: String,
    pub sum: i32,
    pub enabled: bool
}

impl _CollectMuls {
    pub fn _new() -> Self {
        _CollectMuls {
            mul_state: _MulStates::Skip,
            do_state: _DoStates::Skip,
            num1: String::new(),
            num2: String::new(),
            sum: 0,
            enabled: true
        }
    }
    fn _clear_nums(&mut self){
        self.num1.clear();
        self.num2.clear();
    }
    fn _multiply_and_sum_nums(&mut self){
        let n1 = self.num1.parse::<i32>().unwrap();
        let n2 = self.num2.parse::<i32>().unwrap();
        self.sum += n1 * n2;
    }

    pub fn _parse(&mut self, c: char){
        match self.mul_state {
            _MulStates::Skip => {
                if c == 'm' && self.enabled {
                    self.mul_state = _MulStates::M;
                }
            }
            _MulStates::M => {
                if c == 'u'  && self.enabled{
                    self.mul_state = _MulStates::Mu;
                }else{
                    self.mul_state = _MulStates::Skip;
                    self._clear_nums();
                }
            },
            _MulStates::Mu => {
                if c == 'l' && self.enabled {
                    self.mul_state = _MulStates::Mul;
                }else{
                    self.mul_state = _MulStates::Skip;
                    self._clear_nums();
                }
            },
            _MulStates::Mul => {
                if c == '('  && self.enabled{
                    self.mul_state = _MulStates::MulPar;
                }else{
                    self.mul_state = _MulStates::Skip;
                    self._clear_nums();
                }
            },
            _MulStates::MulPar => {
                if c.is_numeric() && self.enabled {
                    self.mul_state = _MulStates::MulParNums;
                    self.num1.push(c);
                }else{
                    self.mul_state = _MulStates::Skip;
                    self._clear_nums();
                }
            },
            _MulStates::MulParNums => {
                if c.is_numeric()  && self.enabled{
                    self.mul_state = _MulStates::MulParNums;
                    self.num1.push(c);
                }else if c == ',' && self.enabled{
                    self.mul_state = _MulStates::MulParNumsC;
                }else{
                    self.mul_state = _MulStates::Skip;
                    self._clear_nums();
                }
            },
            _MulStates::MulParNumsC => {
                if c.is_numeric()  && self.enabled{
                    self.num2.push(c);
                    self.mul_state = _MulStates::MulParNumsCommaNums;
                }else{
                    self.mul_state = _MulStates::Skip;
                    self._clear_nums();
                }
            },
            _MulStates::MulParNumsCommaNums => {
                if c.is_numeric() && self.enabled {
                    self.mul_state = _MulStates::MulParNumsCommaNums;
                    self.num2.push(c);
                }else if c == ')' && self.enabled{
                    self.mul_state = _MulStates::Skip;
                    print!("Multiplying {} * {}\n", self.num1, self.num2);
                    self._multiply_and_sum_nums();
                    self._clear_nums();
                }else{
                    self.mul_state = _MulStates::Skip;
                    self._clear_nums();
                }
            }
        }

        match self.do_state {
            _DoStates::Skip => {
                if c == 'd' {
                    self.do_state = _DoStates::D;
                }
            },
            _DoStates::D => {
                if c == 'o' {
                    self.do_state = _DoStates::Do;
                }else{
                    self.do_state = _DoStates::Skip;
                }
            },
            _DoStates::Do => {
                if c == '(' {
                    self.do_state = _DoStates::DoPar;
                }else if c == 'n' {
                    self.do_state = _DoStates::Don;
                }
                else{
                    self.do_state = _DoStates::Skip;
                }
            },
            _DoStates::DoPar => {
                if c == ')' {
                    self.enabled = true;
                }
                self.do_state = _DoStates::Skip;

            },
            _DoStates::Don => {
                if c == '\'' {
                    self.do_state = _DoStates::Donn;
                }else{
                    self.do_state = _DoStates::Skip;
                }
            },
            _DoStates::Donn => {
                if c == 't' {
                    self.do_state = _DoStates::Donnt;
                }else{
                    self.do_state = _DoStates::Skip;
                }
            },
            _DoStates::Donnt => {
                if c == '(' {
                    self.do_state = _DoStates::DonntPar;
                }else{
                    self.do_state = _DoStates::Skip;
                }
            },
            _DoStates::DonntPar => {
                if c == ')' {
                    self.do_state = _DoStates::Skip;
                    self.enabled = false;
                    print!("Disabling\n");
                }else{
                    self.do_state = _DoStates::Skip;
                }
            }
        }
    }
}

fn _part1_2(){
    // Read from input file
    let input: &str = include_str!("input.txt");
    let mut muls = _CollectMuls::_new();
    for char in input.chars(){
        muls._parse(char);
    }
    print!("Day 3: Part 2: {}\n", muls.sum);
}



