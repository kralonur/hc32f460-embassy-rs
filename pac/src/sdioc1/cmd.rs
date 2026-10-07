#[doc = "Register `CMD` reader"]
pub type R = crate::R<CmdSpec>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `RESTYP` reader - desc RESTYP"]
pub type RestypR = crate::FieldReader;
#[doc = "Field `RESTYP` writer - desc RESTYP"]
pub type RestypW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CCE` reader - desc CCE"]
pub type CceR = crate::BitReader;
#[doc = "Field `CCE` writer - desc CCE"]
pub type CceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICE` reader - desc ICE"]
pub type IceR = crate::BitReader;
#[doc = "Field `ICE` writer - desc ICE"]
pub type IceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAT` reader - desc DAT"]
pub type DatR = crate::BitReader;
#[doc = "Field `DAT` writer - desc DAT"]
pub type DatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYP` reader - desc TYP"]
pub type TypR = crate::FieldReader;
#[doc = "Field `TYP` writer - desc TYP"]
pub type TypW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX` reader - desc IDX"]
pub type IdxR = crate::FieldReader;
#[doc = "Field `IDX` writer - desc IDX"]
pub type IdxW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - desc RESTYP"]
    #[inline(always)]
    pub fn restyp(&self) -> RestypR {
        RestypR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - desc CCE"]
    #[inline(always)]
    pub fn cce(&self) -> CceR {
        CceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc ICE"]
    #[inline(always)]
    pub fn ice(&self) -> IceR {
        IceR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc DAT"]
    #[inline(always)]
    pub fn dat(&self) -> DatR {
        DatR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - desc TYP"]
    #[inline(always)]
    pub fn typ(&self) -> TypR {
        TypR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13 - desc IDX"]
    #[inline(always)]
    pub fn idx(&self) -> IdxR {
        IdxR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc RESTYP"]
    #[inline(always)]
    pub fn restyp(&mut self) -> RestypW<'_, CmdSpec> {
        RestypW::new(self, 0)
    }
    #[doc = "Bit 3 - desc CCE"]
    #[inline(always)]
    pub fn cce(&mut self) -> CceW<'_, CmdSpec> {
        CceW::new(self, 3)
    }
    #[doc = "Bit 4 - desc ICE"]
    #[inline(always)]
    pub fn ice(&mut self) -> IceW<'_, CmdSpec> {
        IceW::new(self, 4)
    }
    #[doc = "Bit 5 - desc DAT"]
    #[inline(always)]
    pub fn dat(&mut self) -> DatW<'_, CmdSpec> {
        DatW::new(self, 5)
    }
    #[doc = "Bits 6:7 - desc TYP"]
    #[inline(always)]
    pub fn typ(&mut self) -> TypW<'_, CmdSpec> {
        TypW::new(self, 6)
    }
    #[doc = "Bits 8:13 - desc IDX"]
    #[inline(always)]
    pub fn idx(&mut self) -> IdxW<'_, CmdSpec> {
        IdxW::new(self, 8)
    }
}
#[doc = "desc CMD\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CmdSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {}
