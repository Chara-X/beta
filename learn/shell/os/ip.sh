# export const ip: {
#     route: {
#         add: (addr: string, dev: string, via?: string) => void
#         del: (addr: string, dev: string) => void
#     }
#     neigh: {
#         add: (addr: string, dev: string, lladdr: string) => void
#         del: (addr: string, dev: string) => void
#     }
#     addr: {
#         add: (addr: string, dev: string) => void
#         del: (addr: string, dev: string) => void
#     }
#     link: {
#         add: (name: string, type: "veth" | "bridge", args?: { peerName: string }) => void
#         del: (name: string) => void
#     }
#     netns: {
#         add: (name: string) => void
#         del: (name: string) => void
#     }
# } = undefined!
# /**
#  * Key: IPDst(addr)
#  * Value: MACDst(via), MACDst(dev)
#  */
