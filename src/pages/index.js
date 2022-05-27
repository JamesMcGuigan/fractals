import Head from 'next/head'
import Mandelbrot from "../components/Mandelbrot";
import FractalExplorer from "../components/FractalExplorer";

export default function Home() {
  return (
    <>
      <Head>
        <title>Fractal Explorer</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main>
          <FractalExplorer>
              <Mandelbrot/>
          </FractalExplorer>
      </main>
    </>
  )
}
