use super::{defs::Bond, molecule::Molecule};

impl Molecule {
    pub fn dfs<F>(&self, start_atom: usize, mut visit: F)
    where
        F: FnMut(usize, Option<&Bond>, &mut bool),
    {
        let mut visited: Vec<bool> = vec![false; self.atoms.len()];
        let mut stack = vec![(start_atom, None)];

        while let Some((current_atom, bond)) = stack.pop() {
            if visited[current_atom] {
                continue;
            }
            let mut skip_visit = false;
            visit(current_atom, bond, &mut skip_visit);
            if skip_visit {
                continue;
            }
            visited[current_atom] = true;

            for &bond_index in &self.atoms[current_atom].outgoing_bond {
                let bond = &self.bonds[bond_index];
                let next_atom = if bond.source == current_atom { bond.dest } else { bond.source };
                if !visited[next_atom] {
                    stack.push((next_atom, Some(bond)));
                }
            }
        }
    }
}
