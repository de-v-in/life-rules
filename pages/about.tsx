import { DefaultLayout } from "@layouts/DefaultLayout";
import { useRouter } from "next/router";

const About: IPageComponent = () => {
  const router = useRouter();
  return (
    <div className="w-full h-full justify-center items-center flex">
      <button
        className="p-3 bg-blue-400 rounded-full active:scale-90 hover:scale-110 duration-300 text-white font-bold"
        onClick={() => router.push("/")}
      >
        GO TO HOME
      </button>
    </div>
  );
};

About.getLayout = (screen) => <DefaultLayout>{screen}</DefaultLayout>;

export default About;
