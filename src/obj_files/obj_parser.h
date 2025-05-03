//
// Created by knightmar on 02/05/25.
//

#ifndef OBJ_PARSER_H
#define OBJ_PARSER_H
#include "../objects/shape.h"

struct {
    char *name;
    SHAPE shape;
} typedef OBJ_FILE;


OBJ_FILE *read_obj_file(const char *filename);



#endif //OBJ_PARSER_H
