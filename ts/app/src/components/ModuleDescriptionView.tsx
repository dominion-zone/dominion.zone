import { Component, JSX, splitProps, Suspense } from "solid-js";
import { ModuleDescription } from "../data/ModuleDescription";

export type ModuleDescriptionProps = JSX.HTMLAttributes<HTMLElement> & {
  packageId: string;
  network: string;
  module: string;
};

const ModuleDescriptionView: Component<ModuleDescriptionProps> = (props) => {
  const [myProps, sectionProps] = splitProps(props, ['network', 'packageId', 'module']);
  const info = ModuleDescription({
    get packageId() {
      return myProps.packageId;
    },
    get network() {
      return myProps.network;
    },
    get module() {
      return myProps.module;
    }
  })
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <section class="card" {...sectionProps}>
        <h2>
          Module: {myProps.module} ({myProps.packageId}) ({myProps.network})
        </h2>
        <p>{info()?.description}</p>
      </section>
    </Suspense>
  )
};

export default ModuleDescriptionView;
