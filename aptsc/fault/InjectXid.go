package fault

import (
	"fmt"

	"golang.org/x/crypto/ssh"
)

func InjectXid(addr, port, path string) {
	c, err := ssh.Dial("tcp", addr, &ssh.ClientConfig{HostKeyCallback: ssh.InsecureIgnoreHostKey(), User: "ubuntu", Auth: []ssh.AuthMethod{ssh.Password("UbuntU1!2@3#4$")}})
	if err != nil {
		panic(err)
	}
	defer c.Close()
	s, err := c.NewSession()
	if err != nil {
		panic(err)
	}
	defer s.Close()
	var cmd string
	for _, v := range map[int]int{1: 132, 2: 133, 3: 134, 4: 135, 5: 136, 6: 137, 7: 138, 8: 139, 9: 141, 10: 142} {
		cmd += fmt.Sprintf("&& /br-dcgmi test --inject --gpuid=%d -f %d -v 8 --host %s:%s", v, v, addr, port)
	}
	if err := s.Run(fmt.Sprintf("sudo -i bash -c 'cd %s %s'", path, cmd)); err != nil {
		panic(err)
	}
}
