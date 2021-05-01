ls -File -Recurse *.rs | Foreach-Object { rustfmt $_ }
