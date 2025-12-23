import { useState } from "react";
import { Card, CardContent } from "@/components/ui/card";
import type { CategoryMinimal } from "@/types";
import { Link } from "react-router-dom";
import { Folder, MoreVertical, Edit2, Trash2 } from "lucide-react";
import { 
  DropdownMenu, 
  DropdownMenuContent, 
  DropdownMenuItem, 
  DropdownMenuTrigger 
} from "@/components/ui/dropdown-menu";
import { 
  AlertDialog, 
  AlertDialogAction, 
  AlertDialogCancel, 
  AlertDialogContent, 
  AlertDialogDescription, 
  AlertDialogFooter, 
  AlertDialogHeader, 
  AlertDialogTitle 
} from "@/components/ui/alert-dialog";
import { 
  Dialog, 
  DialogContent, 
  DialogDescription, 
  DialogHeader, 
  DialogTitle 
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { CategoryForm } from "./CategoryForm";

interface CategoryCardProps {
  category: CategoryMinimal;
  onUpdate?: (id: string, data: { name: string; imageUrl: string; description: string }) => void;
  onDelete?: (id: string) => void;
}

export function CategoryCard({ category, onUpdate, onDelete }: CategoryCardProps) {
  const [isEditDialogOpen, setIsEditDialogOpen] = useState(false);
  const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false);
  const [isLoading, setIsLoading] = useState(false);

  const handleUpdate = (data: { name: string; imageUrl: string; description: string }) => {
    setIsLoading(true);
    // TODO: Implement actual API call
    console.log("Updating category:", category.id, data);
    setTimeout(() => {
      onUpdate?.(category.id, data);
      setIsLoading(false);
      setIsEditDialogOpen(false);
    }, 1000);
  };

  const handleDelete = () => {
    setIsLoading(true);
    // TODO: Implement actual API call
    console.log("Deleting category:", category.id);
    setTimeout(() => {
      onDelete?.(category.id);
      setIsLoading(false);
      setIsDeleteDialogOpen(false);
    }, 1000);
  };

  return (
    <div className="group relative">
      <Link to={`/category/${category.id}`}>
        <Card className="hover:shadow-md transition-shadow cursor-pointer overflow-hidden border-muted h-full">
          <CardContent className="p-6 flex flex-col items-center text-center space-y-4">
            <div className="w-16 h-16 rounded-2xl bg-primary/10 flex items-center justify-center overflow-hidden">
              {category.imageUrl ? (
                <img 
                  src={category.imageUrl} 
                  alt={category.name} 
                  className="w-full h-full object-cover"
                />
              ) : (
                <Folder className="w-8 h-8 text-primary" />
              )}
            </div>
            <div className="space-y-1">
              <h3 className="font-bold text-lg tracking-tight">{category.name}</h3>
              <p className="text-sm text-muted-foreground font-medium">
                {category.taskCount} {category.taskCount === 1 ? 'Task' : 'Tasks'}
              </p>
            </div>
          </CardContent>
        </Card>
      </Link>

      <div className="absolute top-4 right-4 opacity-0 group-hover:opacity-100 transition-opacity">
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="ghost" size="icon" className="h-8 w-8 rounded-full bg-background/80 backdrop-blur shadow-sm">
              <MoreVertical className="h-4 w-4" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end" className="rounded-xl">
            <DropdownMenuItem onClick={() => setIsEditDialogOpen(true)} className="cursor-pointer">
              <Edit2 className="mr-2 h-4 w-4" /> Edit
            </DropdownMenuItem>
            <DropdownMenuItem onClick={() => setIsDeleteDialogOpen(true)} className="cursor-pointer text-destructive focus:text-destructive">
              <Trash2 className="mr-2 h-4 w-4" /> Delete
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>

      {/* Edit Dialog */}
      <Dialog open={isEditDialogOpen} onOpenChange={setIsEditDialogOpen}>
        <DialogContent className="sm:max-w-[425px] rounded-2xl">
          <DialogHeader>
            <DialogTitle className="text-2xl font-bold">Edit Category</DialogTitle>
            <DialogDescription>
              Update the details of your category.
            </DialogDescription>
          </DialogHeader>
          <CategoryForm 
            initialData={{ 
              name: category.name, 
              imageUrl: category.imageUrl, 
              description: category.description 
            }}
            onSubmit={handleUpdate} 
            onCancel={() => setIsEditDialogOpen(false)}
            isLoading={isLoading}
            submitLabel="Update Category"
          />
        </DialogContent>
      </Dialog>

      {/* Delete Confirmation */}
      <AlertDialog open={isDeleteDialogOpen} onOpenChange={setIsDeleteDialogOpen}>
        <AlertDialogContent className="rounded-2xl">
          <AlertDialogHeader>
            <AlertDialogTitle className="text-xl font-bold">Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete the 
              <span className="font-semibold text-foreground"> "{category.name}" </span> 
              category and all its tasks.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter className="gap-2">
            <AlertDialogCancel className="font-medium">Cancel</AlertDialogCancel>
            <AlertDialogAction 
              onClick={(e) => {
                e.preventDefault();
                handleDelete();
              }}
              className="bg-destructive text-destructive-foreground hover:bg-destructive/90 font-semibold"
            >
              {isLoading ? "Deleting..." : "Delete Category"}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  );
}

