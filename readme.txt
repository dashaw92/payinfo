Suite to dump RGIS/WIS paystub event tables into CSVs for querying.

1) Download paystubs from Paperless Employee
2) Run dumptxt.sh against the pdf: ./dumptxt.sh paystub.pdf paystub.txt
3) Run payinfo against the txt: cd payinfo; cargo run -- ../paystub.txt > paystub.csv