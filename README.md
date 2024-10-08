# ASCII-Art generator
This is a simple website that generates ASCII art from a provided image.
- [Check it out here](https://artscii.onrender.com/)
- [And also here a landing page](https://ablaze-event-180.notion.site/ArtSCII-1010169a6c908021a1e9fbb51bcb16d7)

## The technology used to build this website is:
- **[Go](https://golang.org/)**: A programming language created at Google.
- **[Chi](https://go-chi.io/#/)**: A lightweight, idiomatic and composable router for building Go HTTP services.
- **[Tailwind CSS](https://tailwindcss.com/)**: A utility-first CSS framework packed with classes like flex, pt-4, text-center and rotate-90 that can be composed to build any design, directly in your markup.
- **[Bun](https://bun.sh/)**: A fast all-in-one JavaScript/TypeScript packager and bundler.
- **[Vue](https://vuejs.org/)**: A progressive JavaScript framework for building user interfaces.

> [!NOTE]
> This website is still in development and it's continuosly adding new features.

## How to run the website locally
1. Clone the repository:
- Using SSH:
```bash
git clone git@github.com:4ster-light/ascii-converter.git
```
- Using HTTPS:
```bash
git clone https://github.com/4ster-light/ascii-converter.git 
```
2. Change the working directory to the project folder:
```bash
cd ascii-converter
```
3. Run the website:
- Using Make:
> [!WARNING]
> For this you need to have **GO** and **Bun** installed.
```bash
make run
```
- Using Docker:
```bash
docker build -t ascii-converter .
docker run -p 8080:8080 ascii-converter
```
4. Open your browser and navigate to `http://localhost:8080`.
