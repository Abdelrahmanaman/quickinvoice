import {
  ChartLine,
  CheckCircle,
  Hourglass,
  PaperPlaneTilt,
} from "@phosphor-icons/react";
import { Card, CardContent, CardHeader, CardTitle } from "../ui/card";

type CardData = {
  title: string;
  value: string;
  description: string;
  icon: React.ForwardRefExoticComponent<React.SVGProps<SVGSVGElement>>;
  iconClassName?: string;
};
const cardData: CardData[] = [
  {
    title: "Total Revenue",
    value: "$45,231.89",
    description: "+20.1% from last month",
    icon: ChartLine,
    iconClassName: "text-green-500",
  },
  {
    title: "Invoices Sent",
    value: "+2350",
    description: "+180.1% from last month",
    icon: PaperPlaneTilt,
    iconClassName: " text-blue-500",
  },
  {
    title: "Paid Invoices",
    value: "+12,234",
    description: "+19% from last month",
    icon: CheckCircle,
    iconClassName: "text-green-500",
  },
  {
    title: "Pending Invoices",
    value: "+573",
    description: "+201 since last hour",
    icon: Hourglass,
    iconClassName: "text-red-500",
  },
];

function InvoiceStats({
  title,
  value,
  description,
  icon: Icon,
  iconClassName,
}: CardData) {
  return (
    <Card className="grow">
      <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle className="text-xl font-medium">{title}</CardTitle>
        {<Icon className={iconClassName} />}
      </CardHeader>
      <CardContent>
        <div className="text-2xl font-bold">{value}</div>
        <p className="text-muted-foreground text-xs">{description}</p>
      </CardContent>
    </Card>
  );
}

export default function InvoiceStatsList() {
  return (
    <div className="flex flex-wrap gap-2">
      {cardData.map((card, index) => (
        <InvoiceStats key={index} {...card} />
      ))}
    </div>
  );
}
