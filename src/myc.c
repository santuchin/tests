
#
problemas con C:
	funciones f() no son funciones que no toman argumentos, f(void) si. por lo que f(void) -> f(), f() -> f(...)
	i++ es ambiguo y innecesario, simplemente podria usarse i += 1 y el compilador lo traduciria a lo mismo

	los syntaxis para punteros es confusa, en mi remake:
		int * -> @int
		const int * -> @const int
		&x -> @x
		*y -> $y

		se usan simbolos como &, y * que se pueden confundir con AND, y multiplicacion.

	eliminar sugerencias de optimizacion, el compilador casi siempre lo hace mejor:
		inline
		register
		auto

#


def Slice [ @void ptr, Size len ]

Int [Int argc, @@Byte argv] main (

	write [stdout, @hello, sizeof hello];

	0
)
