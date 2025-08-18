package main

import (
	"context"
	"fmt"
	"os"
	"os/signal"
	"syscall"

	"github.com/nathan-fiscaletti/key-logger/pkg/key"
	"github.com/nathan-fiscaletti/key-logger/pkg/keyboard"
	"github.com/nathan-fiscaletti/key-logger/pkg/listener"
)

func isLizardKey(k key.Key) bool {
	// Check if the key is one of the LIZARD letters
	switch k {
	case key.L, key.I, key.Z, key.A, key.R, key.D:
		return true
	default:
		return false
	}
}

func main() {
	fmt.Println("Starting Lizard keystroke audio player...")
	fmt.Println("Press any letter in 'LIZARD' to trigger audio. Press Ctrl+C to exit.")
	fmt.Println("Running in background...")

	ctx := context.Background()
	ctx, cancel := context.WithCancel(ctx)
	defer cancel()

	// Set up signal handling for graceful shutdown
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)

	// Initialize keyboard and listener
	kb := keyboard.New()
	kl := listener.New(kb)
	
	go func() {
		err := kl.Listen(ctx, func(event listener.KeyEvent) {
			fmt.Printf("Key event: %+v\n", event.Key)
			
			if isLizardKey(event.Key) {
				fmt.Printf("🦎 LIZARD key detected: %+v - would play audio here!\n", event.Key)
			}
		})
		if err != nil {
			fmt.Printf("Error listening for keyboard events: %v\n", err)
		}
	}()

	// Wait for interrupt signal
	<-sigChan
	fmt.Println("\nShutting down...")
	cancel()
	fmt.Println("Exited.")
}