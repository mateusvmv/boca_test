Escrito para testar casos da Maratona SBC.
Uso: boca_test [CONFIG]

Exemplo:
> cat a.toml
```toml
build_cmd = "g++"
build_args = ["main.cpp"]
exe_cmd = "./a.out"
exe_args = [""]
problem_dir = "A"
```
> boca_test a.toml
```
✅ - Compiled in 609.171917ms
✅ - Test A_1 - OK in 1.273069ms
❌ - Test A_10 - Expected '25417', got '29412'
```