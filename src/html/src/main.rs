

fn escape(text: &[u8]) -> Vec<u8> {

	let mut result = Vec::with_capacity(
		text.len() +
		text.iter().filter(|byte| b"&<>'".contains(*byte)).count() * 5
	);

	for byte in text {

		match byte {
			b'&' => result.extend_from_slice(b"&#38;"),
			b'<' => result.extend_from_slice(b"&#60;"),
			b'>' => result.extend_from_slice(b"&#62;"),
			b'\'' => result.extend_from_slice(b"&#39;"),
			_ => result.push(*byte),
		}
	}

	result
}


fn write_meta(
	html: &mut Vec<u8>,
	name: &[u8],
	content: &[u8],
) {
	result.extend_from_slice(b"<meta name=\"");
	result.extend_from_slice(name);
	result.extend_from_slice(b"\" content=\"");
	result.extend(escape(value));
	result.extend_from_slice(b"\">");
}


fn generate(
	charset: Option<&[u8]>,
	viewport: Option<&[u8]>,
	title: Option<&[u8]>,
	icon: Option<&[u8]>
) -> Vec<u8> {

	let mut result = Vec::new();

	result.extend_from_slice(b"<!DOCTYPE html>");

	result.extend_from_slice(b"<html>");

	result.extend_from_slice(b"<head>");

	/*
	<head>
		<!-- 1. Codificación y compatibilidad -->
		<meta charset="UTF-8">
		<meta name="viewport" content="width=device-width, initial-scale=1.0">

		<!-- 2. Información básica / SEO -->
		<title>Mi Página Perfecta</title>
		<meta name="description" content="Descripción corta y clara de la página">
		<meta name="keywords" content="palabra1, palabra2, palabra3">
		<meta name="author" content="Tu Nombre">
		<meta name="robots" content="index, follow">

		<!-- 3. Recursos visuales y PWA -->
		<link rel="icon" href="favicon.ico">
		<link rel="manifest" href="site.webmanifest">
		<meta name="theme-color" content="#336699">

		<!-- 4. Estilos -->
		<link rel="stylesheet" href="styles.css">

		<!-- 5. Scripts no bloqueantes -->
		<script src="script.js" defer></script>
	</head>
	*/



	if let Some(value) = charset {
		result.extend_from_slice(b"<meta charset=\">");
		result.extend(escape(value));
		result.extend_from_slice(b"\">");
	}

	if let Some(value) = viewport { write_meta(&mut result, b"viewport", value); }

	if let Some(value) = title {
		result.extend_from_slice(b"<title>");
		result.extend(escape(value));
		result.extend_from_slice(b"</title>");
	}

	if let Some(value) = description { write_meta(&mut result, b"description", value); }
	if let Some(value) = keywords { write_meta(&mut result, b"keywords", value); }
	if let Some(value) = author { write_meta(&mut result, b"author", value); }
	if let Some(value) = robots { write_meta(&mut result, b"robots", value); }

	if let Some(value) = icon {
		result.extend_from_slice(b"<link rel=\"icon\" href=\"");
		result.extend(escape(value));
		result.extend_from_slice(b"\">");
	}

	if let Some(value) = theme_color { write_meta(&mut result, b"theme-color", value); }



	result.extend_from_slice(b"</head>");

	result.extend_from_slice(b"<body>");
	result.extend_from_slice(b"</body>");

	result.extend_from_slice(b"</html>");


	result
}



fn main() {
	
	let html = generate(
		Some(b"UTF-8"),
		Some(b"width=device-width, initial-scale=1.0"),
		Some(b"hello, world"),
	);

	dbg!(std::str::from_utf8(&html).unwrap());
}
