import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";

interface CategoryFormProps {
  initialData?: {
    name: string;
    imageUrl?: string | null;
    description?: string | null;
  };
  onSubmit: (data: { name: string; imageUrl: string; description: string }) => void;
  onCancel: () => void;
  isLoading?: boolean;
  submitLabel: string;
}

export function CategoryForm({ 
  initialData, 
  onSubmit, 
  onCancel, 
  isLoading, 
  submitLabel 
}: CategoryFormProps) {
  const [name, setName] = useState(initialData?.name || "");
  const [imageUrl, setImageUrl] = useState(initialData?.imageUrl || "");
  const [description, setDescription] = useState(initialData?.description || "");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit({ name, imageUrl: imageUrl || "", description: description || "" });
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      <div className="space-y-4">
        <div className="space-y-2">
          <Label htmlFor="name" className="text-sm font-semibold">Category Name</Label>
          <Input
            id="name"
            placeholder="e.g. Work, Home, Fitness"
            required
            value={name}
            onChange={(e) => setName(e.target.value)}
            disabled={isLoading}
            className="h-11 shadow-sm"
          />
        </div>
        <div className="space-y-2">
          <Label htmlFor="imageUrl" className="text-sm font-semibold">Image URL (Optional)</Label>
          <Input
            id="imageUrl"
            placeholder="https://example.com/image.png"
            value={imageUrl || ''}
            onChange={(e) => setImageUrl(e.target.value)}
            disabled={isLoading}
            className="h-11 shadow-sm"
          />
        </div>
        <div className="space-y-2">
          <Label htmlFor="description" className="text-sm font-semibold">Description (Optional)</Label>
          <Textarea
            id="description"
            placeholder="Brief description of this category..."
            value={description || ''}
            onChange={(e) => setDescription(e.target.value)}
            disabled={isLoading}
            className="resize-none shadow-sm min-h-[100px]"
          />
        </div>
      </div>
      <div className="flex flex-col-reverse sm:flex-row sm:justify-end gap-3 pt-4">
        <Button 
          type="button" 
          variant="outline" 
          onClick={onCancel} 
          disabled={isLoading}
          className="font-medium"
        >
          Cancel
        </Button>
        <Button 
          type="submit" 
          disabled={isLoading}
          className="font-semibold px-6 shadow-sm"
        >
          {isLoading ? "Saving..." : submitLabel}
        </Button>
      </div>
    </form>
  );
}
