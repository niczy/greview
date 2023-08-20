import { spawn, ChildProcess } from 'child_process';
import * as http from 'http';
import request from 'sync-request';


export async function startServer(): Promise<ChildProcess> {
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

  for (var i = 0; i < 10; i++) {
    const url = 'http://127.0.0.1:8085';
    const response = await fetch(url);
    if (response.status != 200) {
      console.log("retry in 1 second.")
      Bun.sleepSync(1000); 
    } else {
      break;
    }
  }
  return childProcess;
}
