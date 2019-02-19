use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let info = Info::new(&config.filename)?;

    match config.output {
        Output::All => info.print_all(),
        Output::Sim => info.print_sim_params(),
        Output::Hubble => info.print_hubble_params(),
        Output::Time => info.print_time_params(),
        Output::Unit => info.print_unit_params(),
        Output::Help => print_help(),
    };

    Ok(())
}

pub enum Output {
    All,
    Sim,
    Hubble,
    Time,
    Unit,
    Help,
}

pub struct Config {
    pub snapnum: u32,
    pub filename: String,
    pub query: String,
    pub output: Output,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let snapnum = match args.next() {
            Some(arg) => match arg.parse::<u32>() {
                Ok(num) => num,
                Err(_) => return Err("[SNAPNUM] must be integer!"),
            },
            None => return Err("Incorrect arguments: [SNAPNUM] [QUERY]"),
        };
        let mut filename = String::from("output_");
        let suffix = format!("{:05}/info_{:05}.txt", snapnum, snapnum);
        filename.push_str(&suffix);

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Incorrect arguments: [SNAPNUM] [QUERY]"),
        };

        let output = match &query[..] {
            "all" => Output::All,
            "sim" => Output::Sim,
            "hub" => Output::Hubble,
            "time" => Output::Time,
            "unit" => Output::Unit,
            "help" => Output::Help,
            _ => {
                println!("Query: {} not recognised!", query);
                Output::All
            }
        };

        Ok(Config {
            snapnum,
            filename,
            query,
            output,
        })
    }
}

fn print_help() {
    println!("rinfo help:");
    println!("Usage: rinfo [SNAPNUM] [QUERY]");
    println!("SNAPNUM: ramses snapshot number");
    println!("QUERY: one of [all/sim/hub/time/unit/help]");
    println!("Note: should be run in directory containing output_XXXXX\n");
}

struct Info {
    ncpu: u32,
    ndim: u32,
    levelmin: u32,
    levelmax: u32,
    ngridmax: u32,
    nstep_coarse: u32,
    aexp: f64,
    time: f64,
    h0: f64,
    omega_m: f64,
    omega_l: f64,
    omega_k: f64,
    omega_b: f64,
    unit_l: f64,
    unit_d: f64,
    unit_t: f64,
    redshift: f64,
    smallh: f64,
    boxsize: f64,
    hz: f64,
}

impl Info {
    fn new(filename: &String) -> Result<Info, Box<dyn Error>> {
        let contents = fs::read_to_string(&filename)?;
        let mut buffer = Vec::new();

        for line in contents.lines() {
            buffer.push(line);
        }
        println!("Opened: {}\n", &filename);

        let ncpu: u32 = Info::extract(buffer[0]).parse()?;
        let ndim: u32 = Info::extract(buffer[1]).parse()?;
        let levelmin: u32 = Info::extract(buffer[2]).parse()?;
        let levelmax: u32 = Info::extract(buffer[3]).parse()?;
        let ngridmax: u32 = Info::extract(buffer[4]).parse()?;
        let nstep_coarse: u32 = Info::extract(buffer[5]).parse()?;

        let boxlen: f64 = Info::extract(buffer[7]).parse()?;
        let time: f64 = Info::extract(buffer[8]).parse()?;
        let aexp: f64 = Info::extract(buffer[9]).parse()?;
        let h0: f64 = Info::extract(buffer[10]).parse()?;
        let omega_m: f64 = Info::extract(buffer[11]).parse()?;
        let omega_l: f64 = Info::extract(buffer[12]).parse()?;
        let omega_k: f64 = Info::extract(buffer[13]).parse()?;
        let omega_b: f64 = Info::extract(buffer[14]).parse()?;
        let unit_l: f64 = Info::extract(buffer[15]).parse()?;
        let unit_d: f64 = Info::extract(buffer[16]).parse()?;
        let unit_t: f64 = Info::extract(buffer[17]).parse()?;

        let redshift = Info::find_redshift(aexp);
        let smallh = h0 / 100.0;
        let boxsize = Info::find_boxsize(unit_l, boxlen);
        let hz = Info::find_hz(aexp, h0, omega_m, omega_k, omega_l);

        Ok(Info {
            ncpu,
            ndim,
            levelmin,
            levelmax,
            ngridmax,
            nstep_coarse,
            aexp,
            time,
            h0,
            omega_m,
            omega_l,
            omega_k,
            omega_b,
            unit_l,
            unit_d,
            unit_t,
            redshift,
            smallh,
            boxsize,
            hz,
        })
    }

    fn extract(buf: &str) -> &str {
        &buf[14..].trim()
    }

    fn find_redshift(aexp: f64) -> f64 {
        1.0 / aexp - 1.0
    }

    fn find_boxsize(unit_l: f64, boxlen: f64) -> f64 {
        boxlen * unit_l / (3.08 * 10.0f64.powf(24.0))
    }

    fn find_hz(aexp: f64, h0: f64, omega_m: f64, omega_k: f64, omega_l: f64) -> f64 {
        let ez = omega_m / aexp.powf(3.0) + omega_k / aexp.powf(2.0) + omega_l;
        h0 * ez.sqrt()
    }

    fn print_all(&self) {
        self.print_sim_params();
        self.print_hubble_params();
        self.print_time_params();
        self.print_unit_params();
    }

    fn print_sim_params(&self) {
        println!("ncpu | ndim | levelmin | levelmax | ngridmax | nstep_coarse");
        println!(
            "{:<4} | {:<4} | {:<8} | {:<8} | {:<8} | {:<12}\n",
            self.ncpu, self.ndim, self.levelmin, self.levelmax, self.ngridmax, self.nstep_coarse
        );
    }

    fn print_hubble_params(&self) {
        println!("h0    | H0 [km/s/Mpc] | H(z) [km/s/Mpc] | omega_m | omega_l | omega_k | omega_b");
        println!(
            "{:<5.3} | {:<13.1} | {:<15.1} | {:<7.3} | {:<7.3} | {:<7.3} | {:<7.3}\n",
            self.smallh, self.h0, self.hz, self.omega_m, self.omega_l, self.omega_k, self.omega_b
        );
    }

    fn print_time_params(&self) {
        println!("time   | a     | z    ");
        println!(
            "{:<6.3} | {:<5.3} | {:<5.3}\n",
            self.time, self.aexp, self.redshift
        );
    }

    fn print_unit_params(&self) {
        println!("boxsize [pMpc] | unit_l  | unit_d  | unit_t ");
        println!(
            "{:<14.1e} | {:<7.1e} | {:<7.1e} | {:<7.1e}\n",
            self.boxsize, self.unit_l, self.unit_d, self.unit_t
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_extract() {
        let test_string = String::from("boxlen      =  0.100000000000000E+01");
        assert_eq!(Info::extract(&test_string), "0.100000000000000E+01");
    }

}
