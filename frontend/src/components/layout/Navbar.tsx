import { Link } from "react-router-dom";
import { ListTodo, User, Settings, LogOut, UserCircle } from "lucide-react";
import { Button } from "@/components/ui/button";
import { 
  DropdownMenu, 
  DropdownMenuContent, 
  DropdownMenuItem, 
  DropdownMenuLabel, 
  DropdownMenuSeparator, 
  DropdownMenuTrigger 
} from "@/components/ui/dropdown-menu";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";

// Mock User Data (Replace with actual auth context later)
const MOCK_USER = {
  displayName: "John Doe",
  imageUrl: "https://github.com/shadcn.png",
  email: "john@example.com"
};

export function Navbar() {
  const handleLogout = () => {
    console.log("Logout clicked");
    // TODO: Implement logout logic
  };

  return (
    <header className="sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div className="container flex h-14 items-center justify-between mx-auto px-4 md:px-8">
        <div className="flex items-center">
          <Link to="/" className="mr-6 flex items-center space-x-2">
            <ListTodo className="h-6 w-6" />
            <span className="font-bold sm:inline-block">Todo List</span>
          </Link>
          <nav className="hidden md:flex items-center space-x-6 text-sm font-medium">
            <Link to="/" className="transition-colors hover:text-foreground/80 text-foreground">
              Dashboard
            </Link>
          </nav>
        </div>

        <div className="flex items-center space-x-4">
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="ghost" className="relative h-10 w-full justify-start gap-2 rounded-full px-2 md:w-auto hover:bg-muted">
                <Avatar className="h-8 w-8">
                  <AvatarImage src={MOCK_USER.imageUrl} alt={MOCK_USER.displayName} />
                  <AvatarFallback>{MOCK_USER.displayName.charAt(0)}</AvatarFallback>
                </Avatar>
                <span className="hidden md:inline-flex font-medium text-sm">
                  {MOCK_USER.displayName}
                </span>
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent className="w-56" align="end" forceMount>
              <DropdownMenuLabel className="font-normal">
                <div className="flex flex-col space-y-1">
                  <p className="text-sm font-medium leading-none">{MOCK_USER.displayName}</p>
                  <p className="text-xs leading-none text-muted-foreground">
                    {MOCK_USER.email}
                  </p>
                </div>
              </DropdownMenuLabel>
              <DropdownMenuSeparator />
              <Link to="/profile">
                <DropdownMenuItem className="cursor-pointer">
                  <UserCircle className="mr-2 h-4 w-4" />
                  <span>Profile</span>
                </DropdownMenuItem>
              </Link>
              <DropdownMenuItem className="cursor-pointer">
                <Settings className="mr-2 h-4 w-4" />
                <span>Preferences</span>
              </DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem className="cursor-pointer text-destructive focus:text-destructive" onClick={handleLogout}>
                <LogOut className="mr-2 h-4 w-4" />
                <span>Log out</span>
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      </div>
    </header>
  );
}

