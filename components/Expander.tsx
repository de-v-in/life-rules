import Accordion, {
  AccordionBody,
  AccordionHeader,
} from "@material-tailwind/react/components/Accordion";
import { useState } from "react";

export const Expander: IComponent<{
  title: string;
  className?: string;
}> = ({ title, children, className = "" }) => {
  const [open, setOpen] = useState(true);
  return (
    <Accordion
      open={open}
      className={`w-full border border-blue-900 ${className} flex flex-col`}
    >
      <AccordionHeader
        className="bg-blue-900 text-white py-1 px-2 text-xs"
        onClick={() => setOpen((prev) => !prev)}
      >
        {title}
      </AccordionHeader>
      <AccordionBody className="py-0 flex-auto h-full relative bg-red-500">
        {children}
      </AccordionBody>
    </Accordion>
  );
};
