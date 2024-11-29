use axum::response::Html;

pub async fn homepage() -> Html<&'static str> {
    Html(
        r##"<!DOCTYPE html>
        <html>
        <head>
            <title>Arisu Tracker</title>
            <script src="https://unpkg.com/htmx.org"></script>
            <script src="https://cdn.tailwindcss.com"></script>
        </head>
        <body class="bg-gray-100 text-gray-900">
            <header class="bg-gray-800 text-white py-4">
                <div class="container mx-auto flex justify-around">
                    <button
                        class="px-4 py-2 rounded bg-gray-700 hover:bg-gray-600"
                        onclick="toggleModule('eureka')">Eureka Weather</button>
                    <button
                        class="px-4 py-2 rounded bg-gray-700 hover:bg-gray-600"
                        onclick="toggleModule('bozja')">Bozja Weather</button>
                </div>
            </header>
            <main class="container mx-auto py-6 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                <div id="eureka" class="bg-white shadow rounded p-4">
                    <h2 class="text-lg font-bold">Eureka Weather</h2>
                    <p>Loading Eureka Weather...</p>
                    <script>
                        fetch('/eureka')
                            .then(response => response.text())
                            .then(html => document.getElementById('eureka').innerHTML = html);
                    </script>
                </div>
                <div id="bozja" class="bg-white shadow rounded p-4">
                    <h2 class="text-lg font-bold">Bozja Weather</h2>
                    <p>Loading Bozja Weather...</p>
                    <script>
                        fetch('/bozja')
                            .then(response => response.text())
                            .then(html => document.getElementById('bozja').innerHTML = html);
                    </script>
                </div>
            </main>
            <script>
                function toggleModule(id) {
                    const module = document.getElementById(id);
                    module.style.display = module.style.display === 'none' ? 'block' : 'none';
                }
            </script>
        </body>
        </html>"##,
    )
}
