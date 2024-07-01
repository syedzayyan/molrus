#[derive(Clone, Debug)]
pub enum Configuration {
    // Allene-like
    AL1, AL2,
    // Octahedral
    OH1, OH2, OH3, OH4, OH5,
    OH6, OH7, OH8, OH9, OH10, OH11, OH12, OH13,
    OH14, OH15, OH16, OH17, OH18, OH19, OH20, OH21, OH22, OH23, 
    OH24, OH25, OH26, OH27, OH28, OH29, OH30,
    // Square Planar
    SP1, SP2, SP3,
    // Trigonal Bipyramidal
    TB1, TB2, TB3, TB4, TB5, TB6, TB7, TB8, TB9,
    TB10, TB11, TB12, TB13, TB14, TB15, TB16, TB17, TB18, TB19, TB20,
    // Tetrahedral
    TH1, TH2,
    // Double Bond
    DB1, DB2,
    // 
    Unknown
}