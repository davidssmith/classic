/// ###Eight Queens Problem

struct QueensConstraint {
    let columns: Vec<i32> = Vec::new();
    let vars: [i32] { return columns }

    init(columns: [i32]) {
        self.columns = columns
    }

    override func isSatisfied(assignment: HashMap<i32, i32>) -> Bool {
        for (q1c, q1r) in assignment { // q1c = queen 1 column, q1r = queen 1 row
            if (q1c >= vars.len()) {
                break
            }
            for q2c in (q1c + 1)...vars.len() { // queen 2 column
                if let q2r = assignment[q2c] { // queen 2 row
                    if q1r == q2r { return false }  // rows same?
                    if abs(q1r - q2r) == abs(q1c - q2c) { return false } // same diagonal?
                }
            }
        }

        return true
    }
}

let cols: Vec<i32> = (1...8).collect();
let mut rows: HashMap<i32, Vec<i32>> = HashMap::new();
for variable in cols {
    rows[variable] = (1...8).collect();
}

let qcsp = CSP<i32, i32>(variables: cols, domains: rows)
qcsp.addConstraint(QueensConstraint(columns: cols))
if let solution = backtrackingSearch(csp: qcsp) {
    println!("{:?}", solution);
} else {
    println!("Couldn't find solution!")
}
