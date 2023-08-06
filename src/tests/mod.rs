#[cfg(test)]
mod tests {
    use std::{
        env,
        process::{Command, Output}, vec,
    };

    use crate::run_file;

    #[test]
    fn interpret_block() {
        let output = Command::new("cargo")
        .arg("run")
        .arg("./src/tests/cases/block.bolt")
        .output()
        .unwrap();
        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "3");
        assert_eq!(lines[1], "3");  
    }

    #[test]
    fn interpret_while() {
        let output = Command::new("cargo")
        .arg("run")
        .arg("./src/tests/cases/while.bolt")
        .output()
        .unwrap();
        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "1");
        assert_eq!(lines[1], "0");
    }

    #[test]
    fn interpret_while_math() {
        let output = Command::new("cargo")
        .arg("run")
        .arg("./src/tests/cases/whilemath.bolt")
        .output()
        .unwrap();
        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 11);
        assert_eq!(lines[0], "10");
        assert_eq!(lines[1], "90");
        assert_eq!(lines[2], "720");
        assert_eq!(lines[3], "5040");
        assert_eq!(lines[4], "30240");
        assert_eq!(lines[5], "151200");
        assert_eq!(lines[6], "604800");
        assert_eq!(lines[7], "1814400");
        assert_eq!(lines[8], "3628800");
        assert_eq!(lines[9], "3628800");
    }

    #[test]
    fn interpret_for() {
        let output = Command::new("cargo")
        .arg("run")
        .arg("./src/tests/cases/forloop.bolt")
        .output()
        .unwrap();
    // println!("in for statement");
        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 22);
        let mut fibo = vec![];
        let mut a = 0;
        let mut b = 1;
        let mut temp;
        for _i in 0..21 {
            fibo.push(a);
            temp = b;
            b = a + b;
            a = temp;
        }

        assert_eq!(lines.len(), fibo.len() + 1);
        for i in 0..fibo.len() {
            assert_eq!(lines[i], fibo[i].to_string());
        }
    }

    #[test]
    fn interpret_fun() {
        let output = Command::new("cargo")
        .arg("run")
        .arg("./src/tests/cases/fundef.bolt")
        .output()
        .unwrap();
    // println!("in for statement");
        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 4, "Output: '{}'", lines.join("\n"));
        assert_eq!(lines[0], "1");
        assert_eq!(lines[1], "2");
        assert_eq!(lines[2], "3");
    }

    #[test]
    fn interpret_fun_local() {
        let output = Command::new("cargo")
        .arg("run")
        .arg("./src/tests/cases/fundef_local.bolt")
        .output()
        .unwrap();
    // println!("in for statement");
        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 2, "Output: '{}'", lines.join("\n"));
        assert_eq!(lines[0], "3");
    }

    #[test]
    fn interpret_fun_return() {
        let output = Command::new("cargo")
        .arg("run")
        .arg("./src/tests/cases/funreturn.bolt")
        .output()
        .unwrap();
    // println!("in for statement");
        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 2, "Output: '{}'", lines.join("\n"));
        assert_eq!(lines[0], "5");
    }

    #[test]
    fn interpret_fun_noreturn() {
        let output = Command::new("cargo")
        .arg("run")
        .arg("./src/tests/cases/funnoreturn.bolt")
        .output()
        .unwrap();
    // println!("in for statement");
        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 4, "Output: '{}'", lines.join("\n"));
        assert_eq!(lines[0], "1");
        assert_eq!(lines[1], "2");
        assert_eq!(lines[2], "nil");
    }

    #[test]
    fn interpret_fun_condreturn() {
        let output = Command::new("cargo")
        .arg("run")
        .arg("./src/tests/cases/funcondreturn.bolt")
        .output()
        .unwrap();
    // println!("in for statement");
        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 5, "Output: '{}'", lines.join("\n"));
        assert_eq!(lines[0], "3");
        assert_eq!(lines[1], "2");
        assert_eq!(lines[2], "1");
        assert_eq!(lines[3], "0");
    }

    #[test]
    fn interpret_fun_verynested() {
        let output = Command::new("cargo")
        .arg("run")
        .arg("./src/tests/cases/funverynest.bolt")
        .output()
        .unwrap();
    // println!("in for statement");
        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 3, "Output: '{}'", lines.join("\n"));
        assert_eq!(lines[0], "1");
        assert_eq!(lines[1], "1");
    }

    #[test]
    fn interpret_fun_closure() {
        let output = Command::new("cargo")
        .arg("run")
        .arg("./src/tests/cases/funclosure.bolt")
        .output()
        .unwrap();
    // println!("in for statement");
        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 5, "Output: '{}'", lines.join("\n"));
        assert_eq!(lines[0], "1");
        assert_eq!(lines[1], "2");
        assert_eq!(lines[2], "1");
        assert_eq!(lines[3], "2");
    }

    #[test]
    fn interpret_fun_anon() {
        let output = Command::new("cargo")
        .arg("run")
        .arg("./src/tests/cases/funanon.bolt")
        .output()
        .unwrap();
    // println!("in for statement");
        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 4, "Output: '{}'", lines.join("\n"));
        assert_eq!(lines[0], "1");
        assert_eq!(lines[1], "2");
        assert_eq!(lines[2], "3");
    }

    #[test]
    fn interpret_fun_anon2() {
        let output = Command::new("cargo")
        .arg("run")
        .arg("./src/tests/cases/funanon2.bolt")
        .output()
        .unwrap();
    // println!("in for statement");
        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 2, "Output: '{}'", lines.join("\n"));
        assert_eq!(lines[0], "1");
    }
}
