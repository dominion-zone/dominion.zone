import {Suspense, type Component} from 'solid-js';
import {A, useLocation} from '@solidjs/router';
import AppBar from './components/AppBar';
import Sidebar from './components/Sidebar';

const App: Component = (props: {children: Element}) => {
  const location = useLocation();

  return (
    <>
      <AppBar />
      <div class="container">
        <Sidebar />
        <main>
          <Suspense>{props.children}</Suspense>
        </main>
      </div>
    </>
  );
};

export default App;
