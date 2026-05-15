import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "HealRoute",
  description: "HealRoute internal web portal",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
