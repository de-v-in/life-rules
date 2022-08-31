import styles from "./styles.module.scss";

export const TemplateComponent: IComponent<{
  color?: string;
  text: string;
}> = ({ color = "black", text }) => {
  return (
    <h1 className={styles.text} style={{ color }}>
      {text}
    </h1>
  );
};
