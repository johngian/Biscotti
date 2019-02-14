# biscotti
Shell command runner with environment variable matrix expansion

Pretty much an overhyped, safe and secure way to run:
```
echo FOO={1,2,3}" "BAR={a,b,c}" "BAZ={asdf,xyz}"\n" | xargs -P 4 -i bash -c "{}; echo \$FOO+\$BAR+\$BAZ"
```
