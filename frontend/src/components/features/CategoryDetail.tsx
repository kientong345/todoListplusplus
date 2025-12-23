import { Card, CardContent } from "@/components/ui/card";

import { Progress } from "@/components/ui/progress";
import { Folder } from "lucide-react";

interface CategoryDetailProps {
  category: {
    id: string;
    name: string;
    imageUrl?: string | null;
    description?: string | null;
    taskCount: number;
    openedTaskCount: number;
    canceledTaskCount: number;
    doneTaskCount: number;
    progress: number;
  };
}

export function CategoryDetail({ category }: CategoryDetailProps) {
  return (
    <Card className="border-none shadow-sm bg-muted/30 mb-8">
      <CardContent className="p-6">
        <div className="flex flex-col md:flex-row gap-6 items-start md:items-center">
          <div className="w-20 h-20 rounded-2xl bg-white shadow-sm flex items-center justify-center shrink-0">
            {category.imageUrl ? (
              <img src={category.imageUrl} alt={category.name} className="w-full h-full object-cover rounded-2xl" />
            ) : (
              <Folder className="w-10 h-10 text-primary" />
            )}
          </div>
          
          <div className="flex-1 space-y-4 w-full">
            <div>
              <h1 className="text-3xl font-extrabold tracking-tight">{category.name}</h1>
              {category.description && (
                <p className="text-muted-foreground mt-1">{category.description}</p>
              )}
            </div>

            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              <div className="bg-background rounded-lg p-3 shadow-sm border">
                <p className="text-xs text-muted-foreground font-medium uppercase">Total Tasks</p>
                <p className="text-2xl font-bold">{category.taskCount}</p>
              </div>
              <div className="bg-background rounded-lg p-3 shadow-sm border">
                <p className="text-xs text-blue-500 font-medium uppercase">Opened</p>
                <p className="text-2xl font-bold">{category.openedTaskCount}</p>
              </div>
              <div className="bg-background rounded-lg p-3 shadow-sm border">
                <p className="text-xs text-green-500 font-medium uppercase">Done</p>
                <p className="text-2xl font-bold">{category.doneTaskCount}</p>
              </div>
              <div className="bg-background rounded-lg p-3 shadow-sm border">
                <p className="text-xs text-red-500 font-medium uppercase">Canceled</p>
                <p className="text-2xl font-bold">{category.canceledTaskCount}</p>
              </div>
            </div>

            <div className="space-y-2">
              <div className="flex justify-between text-sm font-medium">
                <span>Progress</span>
                <span>{Math.round(category.progress)}%</span>
              </div>
              <Progress value={category.progress} className="h-2" />
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
