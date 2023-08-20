import { spawn, ChildProcess } from 'child_process';


export function startServer(): ChildProcess {
  // Define the command and its arguments
  const command = 'cargo';
  const args = ['run'];

  // Define environment variables
  const envVars = {
      RUST_LOG: 'debug',
    };

  // Create a child process
  const childProcess = spawn(command, args, {
    detached: true, // Run the process in the background
    stdio: 'inherit', // Redirect stdout and stderr to the current terminal
    env: {
      ...process.env, // Pass the current environment variables if needed
      ...envVars,     // Add your custom environment variables here
    },
  });

  // Handle process exit
  childProcess.on('exit', (code, signal) => {
    if (code !== null) {
      console.log(`Child process exited with code ${code}`);
    } else if (signal !== null) {
      console.log(`Child process was killed with signal ${signal}`);
    }
  });

  // Detach from the child process
  // childProcess.unref();
  return childProcess;
}