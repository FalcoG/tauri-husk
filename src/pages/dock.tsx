import {getCurrent, PhysicalSize, WebviewWindow} from '@tauri-apps/api/window'
import {useEffect, useLayoutEffect, useState} from "react";

// async function resize() {
//   const window = getCurrent();
//   const size = await window.innerSize();
//   await window.setSize(new PhysicalSize(size.width, size.height));
// }

function App() {
  // const [applicationWindow, setApplicationWindow] = useState<WebviewWindow>(undefined)
  //
  // useEffect(() => {
  //   if (window === undefined) return
  //   const currentWindow = getCurrent()
  //   setApplicationWindow(currentWindow)
  // }, [])
  //
  // useLayoutEffect(() => {
  //   if (applicationWindow === undefined) return
  //
  //   console.log('layout effect')
  //   applicationWindow.innerSize().then(size => {
  //     console.log('set size', size)
  //     void applicationWindow.setSize(new PhysicalSize(size.width, size.height));
  //   })
  // }, [applicationWindow])

  // async function testFN() {
  //   const current = getCurrent()
  //   const innerSize = await current.innerSize()
  //   console.log('TESTFN', innerSize)
  //   // await current.setSize(innerSize)
  // }

  return (
    <div className="container">
      <h1>Dock</h1>
    </div>
  );
}

/**
 * {
 *         "label": "local",
 *         "title": "Tauri",
 *         "url": "/dock",
 *         "alwaysOnTop": true,
 *         "decorations": false
 *       }
 */
export default App;
