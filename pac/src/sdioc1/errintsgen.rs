#[doc = "Register `ERRINTSGEN` reader"]
pub type R = crate::R<ErrintsgenSpec>;
#[doc = "Register `ERRINTSGEN` writer"]
pub type W = crate::W<ErrintsgenSpec>;
#[doc = "Field `CTOESEN` reader - desc CTOESEN"]
pub type CtoesenR = crate::BitReader;
#[doc = "Field `CTOESEN` writer - desc CTOESEN"]
pub type CtoesenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCESEN` reader - desc CCESEN"]
pub type CcesenR = crate::BitReader;
#[doc = "Field `CCESEN` writer - desc CCESEN"]
pub type CcesenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEBESEN` reader - desc CEBESEN"]
pub type CebesenR = crate::BitReader;
#[doc = "Field `CEBESEN` writer - desc CEBESEN"]
pub type CebesenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIESEN` reader - desc CIESEN"]
pub type CiesenR = crate::BitReader;
#[doc = "Field `CIESEN` writer - desc CIESEN"]
pub type CiesenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOESEN` reader - desc DTOESEN"]
pub type DtoesenR = crate::BitReader;
#[doc = "Field `DTOESEN` writer - desc DTOESEN"]
pub type DtoesenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCESEN` reader - desc DCESEN"]
pub type DcesenR = crate::BitReader;
#[doc = "Field `DCESEN` writer - desc DCESEN"]
pub type DcesenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBESEN` reader - desc DEBESEN"]
pub type DebesenR = crate::BitReader;
#[doc = "Field `DEBESEN` writer - desc DEBESEN"]
pub type DebesenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACESEN` reader - desc ACESEN"]
pub type AcesenR = crate::BitReader;
#[doc = "Field `ACESEN` writer - desc ACESEN"]
pub type AcesenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CTOESEN"]
    #[inline(always)]
    pub fn ctoesen(&self) -> CtoesenR {
        CtoesenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CCESEN"]
    #[inline(always)]
    pub fn ccesen(&self) -> CcesenR {
        CcesenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CEBESEN"]
    #[inline(always)]
    pub fn cebesen(&self) -> CebesenR {
        CebesenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CIESEN"]
    #[inline(always)]
    pub fn ciesen(&self) -> CiesenR {
        CiesenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc DTOESEN"]
    #[inline(always)]
    pub fn dtoesen(&self) -> DtoesenR {
        DtoesenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc DCESEN"]
    #[inline(always)]
    pub fn dcesen(&self) -> DcesenR {
        DcesenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc DEBESEN"]
    #[inline(always)]
    pub fn debesen(&self) -> DebesenR {
        DebesenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - desc ACESEN"]
    #[inline(always)]
    pub fn acesen(&self) -> AcesenR {
        AcesenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CTOESEN"]
    #[inline(always)]
    pub fn ctoesen(&mut self) -> CtoesenW<'_, ErrintsgenSpec> {
        CtoesenW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CCESEN"]
    #[inline(always)]
    pub fn ccesen(&mut self) -> CcesenW<'_, ErrintsgenSpec> {
        CcesenW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CEBESEN"]
    #[inline(always)]
    pub fn cebesen(&mut self) -> CebesenW<'_, ErrintsgenSpec> {
        CebesenW::new(self, 2)
    }
    #[doc = "Bit 3 - desc CIESEN"]
    #[inline(always)]
    pub fn ciesen(&mut self) -> CiesenW<'_, ErrintsgenSpec> {
        CiesenW::new(self, 3)
    }
    #[doc = "Bit 4 - desc DTOESEN"]
    #[inline(always)]
    pub fn dtoesen(&mut self) -> DtoesenW<'_, ErrintsgenSpec> {
        DtoesenW::new(self, 4)
    }
    #[doc = "Bit 5 - desc DCESEN"]
    #[inline(always)]
    pub fn dcesen(&mut self) -> DcesenW<'_, ErrintsgenSpec> {
        DcesenW::new(self, 5)
    }
    #[doc = "Bit 6 - desc DEBESEN"]
    #[inline(always)]
    pub fn debesen(&mut self) -> DebesenW<'_, ErrintsgenSpec> {
        DebesenW::new(self, 6)
    }
    #[doc = "Bit 8 - desc ACESEN"]
    #[inline(always)]
    pub fn acesen(&mut self) -> AcesenW<'_, ErrintsgenSpec> {
        AcesenW::new(self, 8)
    }
}
#[doc = "desc ERRINTSGEN\n\nYou can [`read`](crate::Reg::read) this register and get [`errintsgen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errintsgen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrintsgenSpec;
impl crate::RegisterSpec for ErrintsgenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`errintsgen::R`](R) reader structure"]
impl crate::Readable for ErrintsgenSpec {}
#[doc = "`write(|w| ..)` method takes [`errintsgen::W`](W) writer structure"]
impl crate::Writable for ErrintsgenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERRINTSGEN to value 0"]
impl crate::Resettable for ErrintsgenSpec {}
