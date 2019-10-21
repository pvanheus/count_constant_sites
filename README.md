#### count_constant_sites

Given a FASTA file with a multiple sequence alignment of nucleotides,
this tool counts the sites in the alignment that are constant. The 
output is a line suitable for use in IQTREE's `-fconst`, thus 4 numbers
with commas expressing the count of As, Cs, Gs and Ts.

A constant site is one where the entire column of the alignment
is one nucleotide. This tool is not case sensitive. Only As, Cs,
Ts and Gs are considered (i.e. gaps and ambiguous nucleotides are
not considered).

TODO: 

* extend to work with protein alphabets
