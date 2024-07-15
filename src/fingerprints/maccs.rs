use crate::{core::molecule::Molecule, parsers::daylight::smarts_defs::SmartsPattern};

pub fn gen_maccs(mol: &Molecule) -> Vec<u8>{
    // List of MACCS Keys in SMARTS

    let maccs_keys: Vec<&str> = vec![ 
        "?",  //ISOTOPE
        "[#104,#105,#106,#107,#106,#109,#110,#111,#112]",  //atomic num >103 Not complete
        "[#104]",  //limit the above def"n since the RDKit only accepts up to #104
        "[#32,#33,#34,#50,#51,#52,#82,#83,#84]",  //Group IVa,Va,VIa Rows 4-6 
        "[Ac,Th,Pa,U,Np,Pu,Am,Cm,Bk,Cf,Es,Fm,Md,No,Lr]",  //actinide
        "[Sc,Ti,Y,Zr,Hf]",  //Group IIIB,IVB (Sc...)  
        "[La,Ce,Pr,Nd,Pm,Sm,Eu,Gd,Tb,Dy,Ho,Er,Tm,Yb,Lu]",  //Lanthanide
        "[V,Cr,Mn,Nb,Mo,Tc,Ta,W,Re]",  //Group VB,VIB,VIIB
        "[!#6;!#1]1~*~*~*~1",  //QAAA@1
        "[Fe,Co,Ni,Ru,Rh,Pd,Os,Ir,Pt]",  //Group VIII (Fe...)
        "[Be,Mg,Ca,Sr,Ba,Ra]",  //Group IIa (Alkaline earth)
        "*1~*~*~*~1",  //4M Ring
        "[Cu,Zn,Ag,Cd,Au,Hg]", //Group IB,IIB (Cu..)
        "[#8]~[#7](~[#6])~[#6]",  //ON(C)C
        "[#16]-[#16]",  //S-S
        "[#8]~[#6](~[#8])~[#8]",  //OC(O)O
        "[!#6;!#1]1~*~*~1",  //QAA@1
        "[#6]#[#6]",  //CTC
        "[#5,#13,#31,#49,#81]",  //Group IIIA (B...) 
        "*1~*~*~*~*~*~*~1",  //7M Ring
        "[#14]",  //Si
        "[#6]=[#6](~[!#6;!#1])~[!#6;!#1]",  //C=C(Q)Q
        "*1~*~*~1",  //3M Ring
        "[#7]~[#6](~[#8])~[#8]",  //NC(O)O
        "[#7]-[#8]",  //N-O
        "[#7]~[#6](~[#7])~[#7]",  //NC(N)N
        "[#6]=;@[#6](@*)@*",  //C$=C($A)$A
        "[I]",  //I
        "[!#6;!#1]~[CH2]~[!#6;!#1]",  //QCH2Q
        "[#15]",  //P
        "[#6]~[!#6;!#1](~[#6])(~[#6])~*",  //CQ(C)(C)A
        "[!#6;!#1]~[F,Cl,Br,I]",  //QX
        "[#6]~[#16]~[#7]",  //CSN
        "[#7]~[#16]",  //NS
        "[CH2]=*",  //CH2=A
        "[Li,Na,K,Rb,Cs,Fr]",  //Group IA (Alkali Metal)
        "[#16R]",  //S Heterocycle
        "[#7]~[#6](~[#8])~[#7]",  //NC(O)N
        "[#7]~[#6](~[#6])~[#7]",  //NC(C)N
        "[#8]~[#16](~[#8])~[#8]",  //OS(O)O
        "[#16]-[#8]",  //S-O
        "[#6]#[#7]",  //CTN
        "F",  //F
        "[!#6;!#1;!H0]~*~[!#6;!#1;!H0]",  //QHAQH
        "[!#1;!#6;!#7;!#8;!#9;!#14;!#15;!#16;!#17;!#35;!#53]",  //OTHER
        "[#6]=[#6]~[#7]",  //C=CN
        "Br",  //BR
        "[#16]~*~[#7]",  //SAN
        "[#8]~[!#6;!#1](~[#8])(~[#8])",  //OQ(O)O
        "[!+0]",  //CHARGE  
        "[#6]=[#6](~[#6])~[#6]",  //C=C(C)C
        "[#6]~[#16]~[#8]",  //CSO
        "[#7]~[#7]",  //NN
        "[!#6;!#1;!H0]~*~*~*~[!#6;!#1;!H0]",  //QHAAAQH
        "[!#6;!#1;!H0]~*~*~[!#6;!#1;!H0]",  //QHAAQH
        "[#8]~[#16]~[#8]",  //OSO
        "[#8]~[#7](~[#8])~[#6]",  //ON(O)C
        "[#8R]",  //O Heterocycle
        "[!#6;!#1]~[#16]~[!#6;!#1]",  //QSQ
        "[#16]!:*:*",  //Snot%A%A
        "[#16]=[#8]",  //S=O
        "*~[#16](~*)~*",  //AS(A)A
        "*@*!@*@*",  //A$!A$A
        "[#7]=[#8]",  //N=O
        "*@*!@[#16]",  //A$A!S
        "c:n",  //C%N
        "[#6]~[#6](~[#6])(~[#6])~*",  //CC(C)(C)A
        "[!#6;!#1]~[#16]",  //QS
        "[!#6;!#1;!H0]~[!#6;!#1;!H0]",  //QHQH (&...) SPEC Incomplete
        "[!#6;!#1]~[!#6;!#1;!H0]",  //QQH
        "[!#6;!#1]~[#7]~[!#6;!#1]",  //QNQ
        "[#7]~[#8]",  //NO
        "[#8]~*~*~[#8]",  //OAAO
        "[#16]=*",  //S=A
        "[CH3]~*~[CH3]",  //CH3ACH3
        "*!@[#7]@*",  //A!N$A
        "[#6]=[#6](~*)~*",  //C=C(A)A
        "[#7]~*~[#7]",  //NAN
        "[#6]=[#7]",  //C=N
        "[#7]~*~*~[#7]",  //NAAN
        "[#7]~*~*~*~[#7]",  //NAAAN
        "[#16]~*(~*)~*",  //SA(A)A
        "*~[CH2]~[!#6;!#1;!H0]",  //ACH2QH
        "[!#6;!#1]1~*~*~*~*~1",  //QAAAA@1
        "[NH2]",  //NH2
        "[#6]~[#7](~[#6])~[#6]",  //CN(C)C
        "[C;H2,H3][!#6;!#1][C;H2,H3]",  //CH2QCH2
        "[F,Cl,Br,I]!@*@*",  //X!A$A
        "[#16]",  //S
        "[#8]~*~*~*~[#8]",  //OAAAO
        "[!#6;!#1;!H0]~*~*~[CH2]~*),$([!#6;!#1;!H0;R]1@[R]@[R]@[CH2;R]1),$([!#6;!#1;!H0]~[R]1@[R]@[CH2;R]1)]", //QHAACH2
        "[!#6;!#1;!H0]~*~*~*~[CH2]~*),$([!#6;!#1;!H0;R]1@[R]@[R]@[R]@[CH2;R]1),$([!#6;!#1;!H0]~[R]1@[R]@[R]@[CH2;R]1),$([!#6;!#1;!H0]~*~[R]1@[R]@[CH2;R]1)]", //QHAAACH2A
        "[#8]~[#6](~[#7])~[#6]",  //OC(N)C
        "[!#6;!#1]~[CH3]",  //QCH3
        "[!#6;!#1]~[#7]",  //QN
        "[#7]~*~*~[#8]",  //NAAO
        "*1~*~*~*~*~1",  //5 M ring
        "[#7]~*~*~*~[#8]",  //NAAAO
        "[!#6;!#1]1~*~*~*~*~*~1",  //QAAAAA@1
        "[#6]=[#6]",  //C=C
        "*~[CH2]~[#7]",  //ACH2N
        "[$([R]@1@[R]@[R]@[R]@[R]@[R]@[R]@[R]1),$([R]@1@[R]@[R]@[R]@[R]@[R]@[R]@[R]@[R]1),$([R]@1@[R]@[R]@[R]@[R]@[R]@[R]@[R]@[R]@[R]1),$([R]@1@[R]@[R]@[R]@[R]@[R]@[R]@[R]@[R]@[R]@[R]1),$([R]@1@[R]@[R]@[R]@[R]@[R]@[R]@[R]@[R]@[R]@[R]@[R]1),$([R]@1@[R]@[R]@[R]@[R]@[R]@[R]@[R]@[R]@[R]@[R]@[R]@[R]1),$([R]@1@[R]@[R]@[R]@[R]@[R]@[R]@[R]@[R]@[R]@[R]@[R]@[R]@[R]1)]", //8M Ring or larger. This only handles up to ring sizes of 14
        "[!#6;!#1]~[#8]",  //QO
        "Cl",  //CL
        "[!#6;!#1;!H0]~*~[CH2]~*",  //QHACH2A
        "*@*(@*)@*",  //A$A($A)$A
        "[!#6;!#1]~*(~[!#6;!#1])~[!#6;!#1]",  //QA(Q)Q
        "[F,Cl,Br,I]~*(~*)~*",  //XA(A)A
        "[CH3]~*~*~*~[CH2]~*",  //CH3AAACH2A
        "*~[CH2]~[#8]",  //ACH2O
        "[#7]~[#6]~[#8]",  //NCO
        "[#7]~*~[CH2]~*",  //NACH2A
        "*~*(~*)(~*)~*",  //AA(A)(A)A
        "[#8]!:*:*",  //Onot%A%A
        "[CH3]~[CH2]~*",  //CH3CH2A
        "[CH3]~*~[CH2]~*",  //CH3ACH2A
        "[$([CH3]~*~*~[CH2]~*),$([CH3]~*1~*~[CH2]1)]",  //CH3AACH2A
        "[#7]~*~[#8]",  //NAO
        "[$(*~[CH2]~[CH2]~*),$(*1~[CH2]~[CH2]1)]",  //ACH2CH2A > 1
        "[#7]=*",  //N=A
        "[!#6;R]",  //Heterocyclic atom > 1 (&...) Spec Incomplete
        "[#7;R]",  //N Heterocycle
        "*~[#7](~*)~*",  //AN(A)A
        "[#8]~[#6]~[#8]",  //OCO
        "[!#6;!#1]~[!#6;!#1]",  //QQ
        "?",  //Aromatic Ring > 1
        "*!@[#8]!@*",  //A!O!A
        "*@*!@[#8]",  //A$A!O > 1 (&...) Spec Incomplete
        "[$(*~[CH2]~*~*~*~[CH2]~*),$([R]1@[CH2;R]@[R]@[R]@[R]@[CH2;R]1),$(*~[CH2]~[R]1@[R]@[R]@[CH2;R]1),$(*~[CH2]~*~[R]1@[R]@[CH2;R]1)]", //ACH2AAACH2A
        "[$(*~[CH2]~*~*~[CH2]~*),$([R]1@[CH2]@[R]@[R]@[CH2;R]1),$(*~[CH2]~[R]1@[R]@[CH2;R]1)]",  //ACH2AACH2A
        "[!#6;!#1]~[!#6;!#1]",  //QQ > 1 (&...)  Spec Incomplete
        "[!#6;!#1;!H0]",  //QH > 1
        "[#8]~*~[CH2]~*",  //OACH2A
        "*@*!@[#7]",  //A$A!N
        "[F,Cl,Br,I]",  //X (HALOGEN)
        "[#7]!:*:*",  //Nnot%A%A
        "[#8]=*",  //O=A>1 
        "[!C;!c;R]",  //Heterocycle
        "[!#6;!#1]~[CH2]~*",  //QCH2A>1 (&...) Spec Incomplete
        "[O;!H0]",  //OH
        "[#8]",  //O > 3 (&...) Spec Incomplete
        "[CH3]",  //CH3 > 2  (&...) Spec Incomplete
        "[#7]",  //N > 1
        "*@*!@[#8]",  //A$A!O
        "*!:*:*!:*",  //Anot%A%Anot%A
        "*1~*~*~*~*~*~1",  //6M ring > 1
        "[#8]",  //O > 2
        "[$(*~[CH2]~[CH2]~*),$([R]1@[CH2;R]@[CH2;R]1)]",  //ACH2CH2A
        "*~[!#6;!#1](~*)~*",  //AQ(A)A
        "[C;H3,H4]",  //CH3 > 1
        "*!@*@*!@*",  //A!A$A!A
        "[#7;!H0]",  //NH
        "[#8]~[#6](~[#6])~[#6]",  //OC(C)C
        "[!#6;!#1]~[CH2]~*",  //QCH2A
        "[#6]=[#8]",  //C=O
        "*!@[CH2]!@*",  //A!CH2!A
        "[#7]~*(~*)~*",  //NA(A)A
        "[#6]-[#8]",  //C-O
        "[#6]-[#7]",  //C-N
        "[#8]",  //O>1
        "[C;H3,H4]",  //CH3
        "[#7]",  //N
        "a",  //Aromatic
        "*1~*~*~*~*~*~1",  //6M Ring
        "[#8]",  //O
        "[R]",  //Ring
        "?"  //Fragments  FIX: this can't be done in SMARTS
    ];
    println!("MACCS Keys: {:?}", maccs_keys);
    let mut bit_vect: Vec<u8> = Vec::new();
    for keys in maccs_keys {
        let conv_smarts = SmartsPattern::new(keys);
        let is_match = conv_smarts.match_mol(& mol);
        bit_vect.push(if is_match {1} else {0});
    }
    return bit_vect
}