current_dir := ${CURDIR}
TOP := computer
SOURCES = $(shell find ${current_dir} -name '*.v' ! -name '*_tb.v')
XDC := ${current_dir}/basys3.xdc

TOP := $(strip ${TOP})
TARGET := $(strip ${TARGET})

BUILDDIR := ${current_dir}/build
BOARD_BUILDDIR := ${BUILDDIR}/${TARGET}

DEVICE := xc7a50t_test
BITSTREAM_DEVICE := artix7
PARTNAME := xc7a35tcpg236-1

XDC_CMD := -x ${XDC}

.DELETE_ON_ERROR:

# Build design
all: ${BOARD_BUILDDIR}/${TOP}.bit

${BOARD_BUILDDIR}:
	mkdir -p ${BOARD_BUILDDIR}

${BOARD_BUILDDIR}/${TOP}.eblif: ${SOURCES} ${XDC} ${SDC} ${PCF} | ${BOARD_BUILDDIR}
	cd ${BOARD_BUILDDIR} && "${current_dir}"/synth.sh -t ${TOP} -v ${SOURCES} -d ${BITSTREAM_DEVICE} -p ${PARTNAME} ${XDC_CMD} 2>&1 > /dev/null

${BOARD_BUILDDIR}/${TOP}.net: ${BOARD_BUILDDIR}/${TOP}.eblif
	cd ${BOARD_BUILDDIR} && symbiflow_pack -e ${TOP}.eblif -d ${DEVICE} ${SDC_CMD} 2>&1 > /dev/null

${BOARD_BUILDDIR}/${TOP}.place: ${BOARD_BUILDDIR}/${TOP}.net
	cd ${BOARD_BUILDDIR} && symbiflow_place -e ${TOP}.eblif -d ${DEVICE} ${PCF_CMD} -n ${TOP}.net -P ${PARTNAME} ${SDC_CMD} 2>&1 > /dev/null

${BOARD_BUILDDIR}/${TOP}.route: ${BOARD_BUILDDIR}/${TOP}.place
	cd ${BOARD_BUILDDIR} && symbiflow_route -e ${TOP}.eblif -d ${DEVICE} ${SDC_CMD} 2>&1 > /dev/null

${BOARD_BUILDDIR}/${TOP}.fasm: ${BOARD_BUILDDIR}/${TOP}.route
	cd ${BOARD_BUILDDIR} && symbiflow_write_fasm -e ${TOP}.eblif -d ${DEVICE}

${BOARD_BUILDDIR}/${TOP}.bit: ${BOARD_BUILDDIR}/${TOP}.fasm
	cd ${BOARD_BUILDDIR} && symbiflow_write_bitstream -d ${BITSTREAM_DEVICE} -f ${TOP}.fasm -p ${PARTNAME} -b ${TOP}.bit

download: ${BOARD_BUILDDIR}/${TOP}.bit
	  openocd -f ~/opt/symbiflow/xc7/conda/envs/xc7/share/openocd/scripts/board/digilent_arty.cfg -c "init; pld load 0 ${BOARD_BUILDDIR}/${TOP}.bit; exit";

clean:
	rm -rf ${BUILDDIR}
